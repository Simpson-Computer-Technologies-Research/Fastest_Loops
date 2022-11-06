# Fastest Loops ![Stars](https://img.shields.io/github/stars/Simpson-Computer-Technologies-Research/Fastest_Loops?color=brightgreen) ![Watchers](https://img.shields.io/github/watchers/Simpson-Computer-Technologies-Research/Fastest_Loops?label=Watchers)
![banner](https://user-images.githubusercontent.com/75189508/200192635-8b052e64-585b-4b15-bacb-b1457cd500ab.png)
What are the Fastest loops between Python, Rust, Golang?

# Results
<h3>Python</h3>

```py
 >> While Loop:         0.624s

 >> Counted For Loop:   0.284s

 >> For Loop:           0.173s
```

<h3>Golang</h3>

```py
 >> While Loop:     2.3914ms

 >> Counted Loop:   1.9946ms

 >> Range Loop:     1.9947ms
```

<h3>Rust (with --release)</h3>

```py
 # // ns (nanosecond) is 1 Millisecond / 1,000,000
 
 >> While Loop:         0ns

 >> Counted For Loop:   0ns

 >> For Loop:           0ns

 >> Iter Loop:          0ns
```

<h3>Rust (not with --release)</h3>

```py
 >> While Loop:         48ms

 >> Counted For Loop:   124ms

 >> For Loop:           173ms

 >> Iter Loop:          194ms
```

# Functions

<h3>Python</h3>

```py
# // 10,000,000 Values
data: list[str] = [1 for _ in range(10 ** 7)]

# // While Loop
def while_loop():
    start_time: int = time.time()
    i: int = 0
    while i < len(data):
        i += 1
    print("\nWhile Loop: "+str(time.time() - start_time))


# // Counted loop
def counted_for_loop():
    start_time: int = time.time()
    for i in range(len(data)):
        i += 0
    print("\nCounted For Loop: "+str(time.time() - start_time))


# // Variable Loop
def for_loop():
    start_time: int = time.time()
    for i in data:
        i += 0
    print("\nFor Loop: "+str(time.time() - start_time))
```

<h3>Golang</h3>

```go
// Main function
func main() {
	var data []int = []int{}
	for i := 0; i < 10_000_000; i++ {
		data = append(data, i)
	}
	WhileLoop(data)
	CountedLoop(data)
	RangeLoop(data)
}

// WhileLoop
func WhileLoop(data []int) {
	var (
		startTime time.Time = time.Now()
		i         int       = 0
	)
	for i < len(data) {
		i++
	}
	fmt.Printf("\nWhile Loop: %v\n", time.Since(startTime))
}

// Counted Loop
func CountedLoop(data []int) {
	var startTime time.Time = time.Now()
	for i := 0; i < len(data); i++ {
		i += 0
	}
	fmt.Printf("\nCounted Loop: %v\n", time.Since(startTime))
}

// Range Loop
func RangeLoop(data []int) {
	var startTime time.Time = time.Now()
	for _, v := range data {
		v += 0
	}
	fmt.Printf("\nRange Loop: %v\n", time.Since(startTime))
}
```

<h3>Rust</h3>

```rust
fn main() {
    // Create the array and fill it with numbers
    let mut data: Vec<i64> = vec![];
    for i in 0..10_000_000 {
        data.push(i as i64)
    }

    // Call the functions
    while_loop(&data);
    counted_loop(&data);
    for_loop(&data);
    iter_loop(&data);
}

// Function to get the current time since epoch in milliseconds
fn get_start_time() -> u128 {
    return std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
}

// While Loop
fn while_loop(data: &[i64]) {
    let start_time: u128 = get_start_time();
    let mut i: i64 = 0;
    while i < data.len().try_into().unwrap() {
        i += 1;
    }
    println!("While Loop: {}ms", get_start_time() - start_time);
}

// Counted For Loop 
fn counted_loop(data: &[i64]) {
    let start_time: u128 = get_start_time();
    let mut i: i64 = 0;
    for _ in 1..data.len() {
        i += 1;
    }
    println!("Counted For Loop: {}ms", get_start_time() - start_time);
}

// For Loop
fn for_loop(data: &[i64]) {
    let start_time: u128 = get_start_time();
    let mut i: i64 = 0;
    for _ in data {
        i += 1;
    }
    println!("For Loop: {}ms", get_start_time() - start_time);
}

// Loop using Iter and ForEach
fn iter_loop(data: &[i64]) {
    let start_time: u128 = get_start_time();
    let mut i: i64 = 0;
    data.iter().for_each(|_| {
        i += 1;
    });
    println!("Iter Loop: {}ms", get_start_time() - start_time);
}
```

# License
MIT License

Copyright (c) 2022 Tristan Simpson

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
