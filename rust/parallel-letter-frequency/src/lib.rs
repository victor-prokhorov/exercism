use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[must_use]
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // test bench_large_parallel   ... bench:     256,248 ns/iter (+/- 49,804)
    // frequency_take_by_ref(input, worker_count)
    // test bench_large_parallel   ... bench:     240,081 ns/iter (+/- 49,657)
    // frequency_scope_join_handles(input, worker_count)
    // test bench_large_parallel   ... bench:     251,984 ns/iter (+/- 46,518)
    // frequency_scope_mutex(input, worker_count)
    // test bench_large_parallel   ... bench:     230,594 ns/iter (+/- 37,915)
    // frequency_mpsc(input, worker_count)
    // test bench_large_parallel   ... bench:     264,729 ns/iter (+/- 45,661)
    // frequency_arc_mutex(input, worker_count)
    // test bench_large_parallel   ... bench:     156,465 ns/iter (+/- 9,430)
    // frequency_rayon(input, worker_count)
    // test bench_large_parallel   ... bench:     118,071 ns/iter (+/- 4,552)
    frequency_rayon_chunks(input, worker_count)
}

#[must_use]
pub fn frequency_rayon_chunks(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let v: Vec<_> = input.chunks(input.len() / worker_count + 1).collect();
    let mut partials = vec![HashMap::new(); worker_count.min(input.len()).min(v.len())];
    partials.par_iter_mut().enumerate().for_each(|(i, m)| {
        for s in v[i] {
            for l in s.chars() {
                if l.is_alphabetic() {
                    m.entry(l.to_ascii_lowercase())
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }
    });
    let mut merged = HashMap::new();
    for m in partials {
        for (k, v) in m {
            merged.entry(k).and_modify(|c| *c += v).or_insert(v);
        }
    }
    merged
}

#[must_use]
pub fn frequency_rayon(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut partials = vec![HashMap::new(); worker_count.max(1)];

    let s = input.concat();

    let v: Vec<char> = s.chars().collect();

    let v: Vec<&[char]> = v.chunks(s.len() / worker_count + 1).collect();

    partials.par_iter_mut().enumerate().for_each(|(i, m)| {
        if i < v.len() {
            v[i].iter().for_each(|l| {
                if l.is_alphabetic() {
                    m.entry(l.to_ascii_lowercase())
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            });
        }
    });

    let mut merged = HashMap::new();

    for m in partials {
        for (k, v) in m {
            merged.entry(k).and_modify(|c| *c += v).or_insert(v);
        }
    }

    merged
}

#[must_use]
pub fn frequency_scope_join_handles(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    if input.is_empty() {
        return map;
    }

    let t = min(worker_count, input.len());

    if t == 1 {
        for i in input {
            for j in i.chars() {
                if j.is_alphabetic() {
                    map.entry(j.to_ascii_lowercase())
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }

        return map;
    }

    // we can skip this step if know that the input is relatively homogeneous
    // here we assume that it's not so we want to avoid big difference in chunks
    // because we want to split our workload evenly
    let s = input.concat();

    let batch_len = s.len() / t + 1;

    let v = s.chars().collect::<Vec<char>>();

    let batches = v.chunks(batch_len);

    thread::scope(|s| {
        let mut handles = Vec::with_capacity(t);

        for batch in batches {
            let handle = s.spawn(|| {
                let mut map = HashMap::<char, usize>::new();

                for i in batch.iter() {
                    if i.is_alphabetic() {
                        map.entry(i.to_ascii_lowercase())
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }
                }

                map
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Ok(i) = handle.join() {
                for e in &i {
                    map.entry(*e.0).and_modify(|c| *c += *e.1).or_insert(*e.1);
                }
            }
        }

        map
    })
}

#[must_use]
pub fn frequency_arc_mutex(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.concat();
    if input.is_empty() {
        return HashMap::new();
    }

    let len = input.len();

    let threads_count_to_spawn = min(worker_count, len);

    if threads_count_to_spawn == 1 {
        let mut hash_map = HashMap::new();
        input.chars().for_each(|char| {
            if char.is_alphabetic() {
                hash_map
                    .entry(char.to_ascii_lowercase())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        });

        return hash_map;
    }

    let batch_len = len / threads_count_to_spawn + 1;
    let mut input = input.chars();

    let mut handles = Vec::with_capacity(threads_count_to_spawn);

    let hash_map = Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..threads_count_to_spawn {
        let input: String = input.by_ref().take(batch_len).collect();
        let hash_map = Arc::clone(&hash_map);

        let handle = thread::spawn(move || {
            let mut map = HashMap::<char, usize>::new();

            input.chars().for_each(|char| {
                if char.is_alphabetic() {
                    map.entry(char.to_ascii_lowercase())
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            });

            let mut hash_map = hash_map.lock().unwrap();
            for (k, v) in map {
                hash_map.entry(k).and_modify(|c| *c += v).or_insert(v);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mutex = Arc::try_unwrap(hash_map).unwrap();
    mutex.into_inner().unwrap()
}

#[must_use]
pub fn frequency_scope_mutex(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let t = min(worker_count, input.len());

    if t == 1 {
        let mut map = HashMap::new();
        for i in input {
            for j in i.chars() {
                if j.is_alphabetic() {
                    map.entry(j.to_ascii_lowercase())
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }

        return map;
    }

    let s = input.concat();
    let batch_len = s.len() / t + 1;
    let v = s.chars().collect::<Vec<char>>();

    let batches = v.chunks(batch_len);

    let map = Mutex::new(HashMap::<char, usize>::new());

    thread::scope(|s| {
        for batch in batches {
            s.spawn(|| {
                let mut m = HashMap::new();
                for i in batch.iter() {
                    if i.is_alphabetic() {
                        m.entry(i.to_ascii_lowercase())
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }
                }
                let mut map = map.lock().unwrap();
                for (k, v) in m {
                    map.entry(k).and_modify(|c| *c += v).or_insert(v);
                }
            });
        }
    });

    map.into_inner().unwrap()
}

#[must_use]
pub fn frequency_mpsc(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut hash_map = HashMap::new();

    let input = input.concat();

    if input.is_empty() {
        return hash_map;
    }

    let len = input.len();

    let threads_count_to_spawn = min(worker_count, len);

    if threads_count_to_spawn == 1 {
        input.chars().for_each(|char| {
            if char.is_alphabetic() {
                hash_map
                    .entry(char.to_ascii_lowercase())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        });

        return hash_map;
    }

    let batch_len = len / threads_count_to_spawn + 1;
    let mut input = input.chars();

    let (tx, rx) = mpsc::channel();

    for _ in 0..threads_count_to_spawn {
        let tx = tx.clone();
        let input: String = input.by_ref().take(batch_len).collect();

        thread::spawn(move || {
            let mut hash_map = HashMap::<char, usize>::new();

            input.chars().for_each(|char| {
                if char.is_alphabetic() {
                    hash_map
                        .entry(char.to_ascii_lowercase())
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            });

            tx.send(hash_map).unwrap();
        });
    }

    for _ in 0..threads_count_to_spawn {
        let batch_hash_map = rx.recv().unwrap();
        for (&batch_char, &batch_counter) in &batch_hash_map {
            hash_map
                .entry(batch_char)
                .and_modify(|counter| *counter += batch_counter)
                .or_insert(batch_counter);
        }
    }

    hash_map
}

#[must_use]
pub fn frequency_take_by_ref(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut hash_map = HashMap::new();

    let input = input.concat();

    if input.is_empty() {
        return hash_map;
    }

    let len = input.len();

    let threads_count_to_spawn = min(worker_count, len);

    if threads_count_to_spawn == 1 {
        input.chars().for_each(|char| {
            if char.is_alphabetic() {
                hash_map
                    .entry(char.to_ascii_lowercase())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        });

        return hash_map;
    }

    let mut handles = Vec::with_capacity(threads_count_to_spawn);

    let batch_len = len / threads_count_to_spawn + 1;

    let mut input = input.chars();

    for _ in 0..threads_count_to_spawn {
        let input: String = input.by_ref().take(batch_len).collect();

        let handle = thread::spawn(move || {
            let mut hash_map = HashMap::<char, usize>::new();

            input.chars().for_each(|char| {
                if char.is_alphabetic() {
                    hash_map
                        .entry(char.to_ascii_lowercase())
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            });

            hash_map
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Ok(batch_hash_map) = handle.join() {
            for (&batch_char, &batch_counter) in &batch_hash_map {
                hash_map
                    .entry(batch_char)
                    .and_modify(|counter| *counter += batch_counter)
                    .or_insert(batch_counter);
            }
        }
    }

    hash_map
}
