#[test]
fn atomic_ordering_model() {
    use loom::sync::atomic::{AtomicUsize, Ordering};
    use loom::thread;

    loom::model(|| {
        let x = AtomicUsize::new(0);

        let t1 = thread::spawn({
            let x = &x;
            move || {
                x.store(1, Ordering::Release);
            }
        });

        let t2 = thread::spawn({
            let x = &x;
            move || {
                let _ = x.load(Ordering::Acquire);
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
    });
}
