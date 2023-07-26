use std::thread;
use std::sync::{ Arc, Mutex };

fn main() {
    let compteur = Arc::new(Mutex::new(0));

    let mut threads = Vec::new();

    for i in 1..10 {
        let compteur_reference = Arc::clone(&compteur);
        let thread_courant = thread::spawn(move || {
            let mut num = compteur_reference.lock().unwrap();
            let id = thread::current().id();
            *num += i;

            println!("Value of compteur in thread {:?} : {}", id, *num);
        });
        threads.push(thread_courant);
    }

    for th in threads {
        th.join().unwrap();
    }

    println!("Last value of compteur after thread {}", *compteur.lock().unwrap());



    #[derive(Debug)]
    struct Joueur {
        speudo: String,
        vie: i32,
    }

    let j1 = Joueur {
        speudo: String::from("JJ le boulet"),
        vie: 7
    };

    let j1_mutex = Arc::new(Mutex::new(j1));
    let mut threads_joueur = Vec::new();

    for i in 1..5 {
        let current_j1_moins_mutex_ref = Arc::clone(&j1_mutex);
        let current_thread_moins_vie = thread::spawn(move || {
            let mut j = current_j1_moins_mutex_ref.lock().unwrap();
            j.vie -= 1;

            println!("Une vie en moins : {:?}", j);
        });

        let current_j1_plus_mutex_ref = Arc::clone(&j1_mutex);
        let current_thread_plus_vie = thread::spawn(move || {
            let mut j = current_j1_plus_mutex_ref.lock().unwrap();
            j.vie += 1;

            println!("Deux vie en plus : {:?}", j);
        });
        threads_joueur.push(current_thread_moins_vie);
        threads_joueur.push(current_thread_plus_vie);
    }

    for th in threads_joueur {
        th.join().unwrap();
    }

    println!("Compteur de vie : {:?}", j1_mutex.lock().unwrap());
}
