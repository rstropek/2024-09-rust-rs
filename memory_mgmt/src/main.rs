use std::{cell::RefCell, rc::Rc, sync::RwLock};

struct MyPreciousRing {
    engraving: String,
}

impl MyPreciousRing {
    fn forge() -> Self {
        Self {
            engraving: "One Ring to rule them all".to_string(),
        }
    }
}

mod we_are_all_friends {
    #[derive(Copy, Clone)]
    pub struct MyPreciousRing {
        pub engraving: &'static str,
    }

    impl MyPreciousRing {
        pub fn forge() -> Self {
            Self {
                engraving: "One Ring to rule them all",
            }
        }
    }
}

struct Item {
    name: &'static str,
}

const STING: Item = Item { name: "Sting" };
const ANDURIL: Item = Item { name: "Andúril" };
const PALANTIRI: Item = Item { name: "Palantíri" };

struct CharacterWithStuff<'a> {
    name: &'static str,
    item: &'a Item,
}

fn main() {
    {
        let saurons_ring = MyPreciousRing::forge();
        let gollums_ring = saurons_ring;
        let bilbos_ring = gollums_ring;
        let mut frodos_ring = bilbos_ring;
        let mut samwise_ring = &frodos_ring;
        println!("{}", samwise_ring.engraving);
        println!("{}", frodos_ring.engraving);
        frodos_ring.engraving = "Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.".to_string();

        samwise_ring = &frodos_ring;
        println!("{}", samwise_ring.engraving);
    }

    {
        let saurons_ring = we_are_all_friends::MyPreciousRing::forge();
        let gollums_ring = saurons_ring;
        println!("{}", gollums_ring.engraving);
        println!("{}", saurons_ring.engraving);
    }

    {
        let stuff = vec![STING, ANDURIL, PALANTIRI];
        let frodo = CharacterWithStuff {
            name: "Frodo",
            item: &stuff[0],
        };
        drop(stuff);
        // println!("{}", frodo.name); -> Does not work!
    }

    {
        let saurons_ring = Rc::new(MyPreciousRing::forge());
        //saurons_ring.engraving = "Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.".to_string();
        let frodos_ring = Rc::clone(&saurons_ring);
        println!("The ring has {} owners", Rc::strong_count(&frodos_ring));
        println!("{}", frodos_ring.engraving);
        let samwise_ring = Rc::clone(&frodos_ring);
        println!("The ring has {} owners", Rc::strong_count(&frodos_ring));
        println!("{}", samwise_ring.engraving);
        drop(samwise_ring);
        println!("The ring has {} owners", Rc::strong_count(&frodos_ring));
        drop(frodos_ring);
        println!("The ring has {} owners", Rc::strong_count(&saurons_ring));
        drop(saurons_ring);
    }
    
    {
        let saurons_ring = Rc::new(MyPreciousRing::forge());
        let frodos_ring = Rc::clone(&saurons_ring);
        let frodos_ring_weak = Rc::downgrade(&frodos_ring);
        if let Some(frodos_ring) = frodos_ring_weak.upgrade() {
            println!("{}", frodos_ring.engraving);
        }
    }
    
    {
        // inner mutability pattern
        let saurons_ring = Rc::new(RefCell::new(MyPreciousRing::forge()));
        let frodos_ring = Rc::clone(&saurons_ring);
        println!("The ring has {} owners", Rc::strong_count(&frodos_ring));
        let frodos_ring_borrow = frodos_ring.borrow();
        println!("{}", frodos_ring_borrow.engraving);
        drop(frodos_ring_borrow);
        frodos_ring.borrow_mut().engraving = "Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.".to_string();
        //println!("{}", frodos_ring_borrow.engraving);
        let samwise_ring = Rc::clone(&frodos_ring);
        println!("{}", samwise_ring.borrow().engraving);
    }

    {
        let saurons_ring = Rc::new(RwLock::new(MyPreciousRing::forge()));
        let frodos_ring = Rc::clone(&saurons_ring);
        let frodos_ring_borrow = frodos_ring.read().unwrap();
        println!("{}", frodos_ring_borrow.engraving);
        drop(frodos_ring_borrow);
        frodos_ring.write().unwrap().engraving = "Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.".to_string();
        //println!("{}", frodos_ring_borrow.engraving);
    }
}
