fn main() {
    // mean_med_mod();
    // pig_latin();
    // hash_map_with_structs();
}

fn mean_med_mod() {
    let mut num_list = vec![1, 544, 122, 4, 120, 863, 38277, 987, 239, 38821, 258, 954883, 271772, 3, 5, 7, 9, 0, 1, 1, 2, 4, 7, 8, 9, 0, 3];

    use std::collections::HashMap;
    let mut multiplicity_map = HashMap::new();

    let mut total = 0; 
    for val in &num_list {
        total += val;
        let cur_mult = multiplicity_map.entry(*val).or_insert(0);
        *cur_mult += 1;
    }
    let mean: f64 = total as f64 / num_list.len() as f64;

    let mut mode = (0, 0);
    for (num, mult) in &multiplicity_map {
        if *mult > mode.1 {
            mode = (*num, *mult);
        }
    }

    &num_list.sort();
    let mut median: f64 = num_list[num_list.len() / 2] as f64;
    if num_list.len() % 2 == 0 {
        median = (median + num_list[num_list.len() / 2 - 1] as f64) / 2.0;
    }



    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}, {} time(s)", mode.0, mode.1);
}

fn pig_latin() {
    let sentence = String::from("Hello world My name is Jack Champagne");
    let mut pl_sent = String::from("");
    for word in sentence.split_whitespace() {
        pl_sent.push_str(&eng_to_pl(&word));
        pl_sent.push_str(" ");
    }
    pl_sent.pop();
    println!("{}: {}", sentence, pl_sent);
}

fn eng_to_pl(eng_word: &str) -> String {
    if eng_word.len() == 0 {
        String::from("");
    }

    let mut w_iter = eng_word.chars();
    let first_char = &w_iter.next();

    match first_char {
        Some('a') => format!("{}-hay", eng_word),
        Some('e') => format!("{}-hay", eng_word),
        Some('i') => format!("{}-hay", eng_word),
        Some('o') => format!("{}-hay", eng_word),
        Some('u') => format!("{}-hay", eng_word),
        Some('A') => format!("{}-hay", eng_word),
        Some('E') => format!("{}-hay", eng_word),
        Some('I') => format!("{}-hay", eng_word),
        Some('O') => format!("{}-hay", eng_word),
        Some('U') => format!("{}-hay", eng_word),
        Some(val) => format!("{}{}ay", w_iter.as_str(), val),
        None => String::from("")
    }

    // There must be a better way of doing this, not only that but there must be a better
    // way that also capitalizes correctly as well.
}

fn hash_map_with_structs() {
    struct Employee {
        name: String,
        dept: String
    }

    let mut employee_list: Vec<Employee> = vec![];
    use std::collections::HashMap;
    let mut employee_directory: HashMap<String, Vec<String>> = HashMap::new();
    
    // Add method takes in Employee object
    let nancy = Employee {
        name: String::from("Nancy Drew"),
        dept: String::from("Mystery")
    };

    let fred = Employee {
        name: String::from("Fred Jones"),
        dept: String::from("Mystery")
    };

    let velma = Employee {
        name: String::from("Velma Dinkley"),
        dept: String::from("Mystery")
    };

    let anakin = Employee {
        name: String::from("Anakin Skywalker"),
        dept: String::from("Jedi")
    };

    let obi_wan = Employee {
        name: String::from("Obi-Wan Kenobi"),
        dept: String::from("Jedi"),
    };

    let palpatine = Employee {
        name: String::from("Emperor Palpatine"),
        dept: String::from("Sith")
    };

    let rey = Employee {
        name: String::from("Rey Skywalker"),
        dept: String::from("The Chosen One")
    };

    employee_list.push(nancy);
    employee_list.push(fred);
    employee_list.push(velma);
    employee_list.push(anakin);
    employee_list.push(obi_wan);
    employee_list.push(palpatine);
    employee_list.push(rey);

    for emp in employee_list {
        let dept = employee_directory.entry(emp.dept).or_insert(vec![]);
        dept.push(emp.name);
    }

    for key in employee_directory.keys() {
        println!("{}: {:?}", key, employee_directory.get(key));
        println!("Not on directory: {:?}", employee_directory.get(&String::from("Random dept name")));
    }
}