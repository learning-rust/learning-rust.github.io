---
headless: true

title: See the syntax in action
description: Let's explore a few syntaxes of the language.
---

{{< accordion-vertical-tabs count="3" tabHeight="3.5rem" label="Select feature" >}}
{{< tab label="Data and behavior" >}}

```rust {title="Structs, Impls and Traits"}
struct Person {
    name: String,
    company_name: String,
}

impl Person {
    fn new(name: String, company_name: String) -> Self {
        Self { name, company_name }
    }

    fn intro(&self) -> String {
        format!("I'm {} from {}", self.name, self.company_name)
    }
}

trait Greet {
    const PREFIX: &'static str = "Hello";

    fn greet(&self) -> String {
        format!("{}!", String::from(Self::PREFIX))
    }
}
impl Greet for Person {}

fn main() {
    let steve = Person::new("Steve".to_string(), "Apple".to_string());

    println!("{}", steve.greet()); // Hello!
    println!("{}", steve.intro()); // I'm Steve from Apple
}
```

{{< /tab >}}
{{< tab label="Functional" >}}

```rust {title="Iterators, Filters, and Maps"}
fn main() {
    let marks = ["10", "20", "30", "0.4", "0.5", "invalid"];

    let sum: i32 = marks
        // Iterate through marks
        .iter()
        // Parse string to float, ignore if invalid
        .filter_map(|&s| s.parse::<f64>().ok())
        // Multiply decimals under 1.0 by 100
        .map(|num| if num < 1.0 { num * 100.0 } else { num })
        // Trace the values
        .inspect(|&val| println!("Parsed: {val}"))
        // Cast float to integer
        .map(|num| num as i32)
        // Aggregate
        .sum();

    println!("Final: {sum}"); // 150
}
```

{{< /tab >}}
{{< tab label="Advanced blueprint" >}}

```rust {title="Enums, Generics and HashMap"}
use std::collections::HashMap;

enum Data<K, V> {
    Value(V),
    KeyValue(K, V),
}

fn main() {
    let data: Vec<Data<&str, i32>> = vec![
        Data::Value(10),
        Data::KeyValue("Steve", 20),
        Data::KeyValue("Tom", 30),
        Data::Value(40),
        Data::KeyValue("Mike", 50),
    ];

    let map: HashMap<String, i32> = data
        .into_iter()
        // Pattern matching and destructuring
        .map(|item| match item {
            Data::KeyValue(k, v) => {
                println!("{}: {}", k, v);
                (k.to_string(), v)
            }
            Data::Value(v) => {
                println!("unknown: {}", v);
                ("unknown".to_string(), v)
            }
        })
        // Accumulate items into a map
        .fold(HashMap::new(), |mut map, (key, value)| {
            map.entry(key)
                .and_modify(|existing| *existing += value)
                .or_insert(value);
            map
        });

    println!("Map: {:?}", map); // Map: {"unknown": 50, "Steve": 20, "Tom": 30, "Mike": 50}
}
```

{{< /tab >}}
{{< /accordion-vertical-tabs >}}