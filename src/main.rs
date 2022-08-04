mod nodes;
mod linked_list;
mod map;

fn main() {
    use map::HashMap;
    use map::Map;

    let mut hmap: Box<dyn Map<char, &str>> = Box::new(HashMap::<char, &str>::new());

    let notes = &["do", "re", "mi", "fa", "sol", "la", "ti"];

    for (letter, note) in ('c'..'h').zip(notes) {
        hmap.put(letter, note);
    }

    for (letter, note) in ('c'..'h').zip(notes) {
        // assert_eq!(alist.get_ref(letter), Some(note));
        println!("{:?}", hmap.get(letter));
    }
}

#[test]
fn stack_test() {
    use linked_list::LinkedList;
    use linked_list::Stack;

    let mut st: Box<dyn Stack<i32>> = Box::new(LinkedList::<i32>::new());

    for i in 0..10 {
        st.push(i);
    }

    for i in 0..10 {
        assert_eq!(st.pop(), Some(9-i));
    }
}

#[test]
fn alist_test() {
    use map::ListMap;
    use map::Map;

    let mut alist: Box<dyn Map<char, &str>> = Box::new(ListMap::<char, &str>::new());

    let notes = &["do", "re", "mi", "fa", "sol", "la", "ti"];

    for (letter, note) in ('c'..'h').zip(notes) {
        alist.put(letter, note);
    }

    for (letter, note) in ('c'..'h').zip(notes) {
        assert_eq!(alist.get_ref(letter), Some(note));
    }
}

#[test]
fn hash_map_test() {
    use map::HashMap;
    use map::Map;

    let mut alist: Box<dyn Map<char, &str>> = Box::new(HashMap::<char, &str>::new());

    let notes = &["do", "re", "mi", "fa", "sol", "la", "ti"];

    for (letter, note) in ('c'..'h').zip(notes) {
        alist.put(letter, note);
    }

    for (letter, note) in ('c'..'h').zip(notes) {
        assert_eq!(alist.get_ref(letter), Some(note));
    }
}
