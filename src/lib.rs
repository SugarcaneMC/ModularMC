#![feature(ptr_metadata)]

mod chunk;

use self::chunk::Chunk;
use crossbeam::{
	deque::{Injector, Steal, Stealer, Worker},
	thread::scope
};
use lockfree::set::Set;

pub struct Task {
	x: usize
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Mutation(*mut [u8]);

unsafe impl Sync for Mutation {}

pub fn start() {
	let threads = 12;

	let guard = Set::new();
	let injector = Injector::new();
	(0..1000).for_each(|_| injector.push(Task { x: 0 }));
	let workers: Vec<_> = (0..=threads).map(|_| Worker::new_fifo()).collect();
	let stealers: Vec<_> = workers.iter().map(Worker::stealer).collect();

	let chunk = Chunk::new();

	scope(|scope| {
		for (index, worker) in workers.into_iter().enumerate() {
			scope
				.builder()
				.name(format!("worker-{}", index))
				.spawn(|_| {
					let worker = worker;
					let stealers = stealers.clone();
					unsafe {
						worker_thread(&guard, &worker, &injector, &stealers, &chunk)
					}
				})
				.unwrap();
		}
	})
	.unwrap()
}

pub unsafe fn worker_thread(
	mutability_guard: &Set<Mutation>,
	local_queue: &Worker<Task>,
	global_queue: &Injector<Task>,
	peer_queues: &[Stealer<Task>],
	chunk: &Chunk
) {
	let get_task = || {
		if let Some(task) = local_queue.pop() {
			let block = chunk.block_at(task.x, 0, 0);
			if let Ok(()) = mutability_guard.insert(Mutation(block.block)) {
				return Some(block);
			}
		}

		if let Steal::Success(task) = global_queue.steal_batch_and_pop(local_queue)
		{
			let block = chunk.block_at(task.x, 0, 0);
			if let Ok(()) = mutability_guard.insert(Mutation(block.block)) {
				return Some(block);
			}
		}

		for peer in peer_queues {
			if let Steal::Success(task) = peer.steal() {
				let block = chunk.block_at(task.x, 0, 0);
				if let Ok(()) = mutability_guard.insert(Mutation(block.block)) {
					return Some(block);
				}
			}
		}

		None
	};

	while let Some(task) = get_task() {
		println!("got a task.");

		mutability_guard.remove(&Mutation(task.block));
	}
}
