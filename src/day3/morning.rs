// 20.4
fn say_hello(name: String) {
  println!("hello, {name}");
}

// 20.6
#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

// 20.7
struct Droppable {
  name: &'static str,
}

impl Drop for Droppable {
  fn drop(&mut self) {
    println!("dropping: {}", self.name);
  }
}

// 20.8
#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// A representation of a software package.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    fn as_dependency(&self) -> Dependency {
        Dependency { name: self.name.clone(), version_expression: self.version.clone() }
    }
}

/// A builder for a Package. Use `build()` to create the `Package` itself.
struct PackageBuilder(Package);

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self(Package{ 
          name: name.into(), 
          version: String::new(), 
          authors: Vec::<String>::new(), 
          dependencies: Vec::<Dependency>::new(), 
          language: None
        })
    }

    /// Set the package version.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Set the package authors.
    fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    /// Add an additional dependency.
    fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    /// Set the language. If not set, language defaults to None.
    fn language(mut self, language: Language) -> Self {
        self.0.language = Some(language);
        self
    }

    fn build(self) -> Package {
        self.0
    }
}

// 21.1
#[derive(Debug)]
enum List<T> {
  Element(T, Box<List<T>>),
  Nil,
}

// 21.2
use std::rc::Rc;

pub fn run() {
  // 20.4
  println!("");
  let ss1: String = "hello".into();
  let ss2: String = ss1;
  println!("ss2: {ss2:?}");
  // println!("ss1: {ss1:?}");
  let hello: String = "mara".into();
  say_hello(hello);
  // say_hello(hello);

  // 20.5
  println!("");
  let name = String::from("mara");
  say_hello(name.clone());
  say_hello(name);

  // 20.6
  println!("");
  let p1 = Point(3, 4);
  let p2 = p1; // by copy trait derived
  println!("p1: {p1:?}");
  println!("p2: {p2:?}");

  // 20.7
  println!("");
  let a = Droppable { name: "a" };
  {
    let b = Droppable { name: "b" };
    {
      let c = Droppable { name: "c" };
      let d = Droppable { name: "d" };
      println!("Exiting block B");
    }
    println!("Exiting block A");
  }
  drop(a);
  println!("");

  // 20.8
  let base64 = PackageBuilder::new("base64").version("0.13").build();
  println!("base64: {base64:?}");
  let log =
      PackageBuilder::new("log").version("0.4").language(Language::Rust).build();
  println!("log: {log:?}");
  let serde = PackageBuilder::new("serde")
      .authors(vec!["djmitche".into()])
      .version(String::from("4.0"))
      .dependency(base64.as_dependency())
      .dependency(log.as_dependency())
      .build();
  println!("serde: {serde:?}");

  // 21.1
  println!("");
  let five = Box::new(5);
  println!("five: {}", *five);
  let list: List<i32> = List::Element(3, Box::new(List::Element(5, Box::new(List::Nil))));
  println!("{list:?}");

  // 21.2
  println!("");
  let a = Rc::new(10);
  let b = Rc::clone(&a);
  println!("a: {a}");
  println!("b: {b}");
  println!("count: {}", Rc::strong_count(&a));
}