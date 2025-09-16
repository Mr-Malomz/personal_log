# Personal Log 📝

> A learning project to explore database internals and data persistence patterns in Rust

## What's This All About?

Hey there! 👋 This is my playground for diving deep into how databases actually work under the hood. Sure, I could just use an existing database and call it a day, but where's the fun in that? 

This project is all about **learning by doing** - building a personal logging CLI tool while exploring the nitty-gritty details of data storage, indexing, querying, and all those database concepts that usually feel like magic.

## The Learning Journey 🚀

### What I'm Exploring

- **Database Fundamentals**: How data gets stored, indexed, and retrieved
- **SQLite Internals**: What happens when you `INSERT` or `SELECT`?
- **Data Serialization**: Moving between memory, disk, and different formats
- **CLI Design**: Building intuitive command-line interfaces
- **Rust Ecosystem**: Getting hands-on with awesome crates


## What This Thing Actually Does

Once it's built out, this will be a simple personal logging tool where you can:

```bash
# Add a quick thought or note
cargo run add "Just learned about B-trees and my brain hurts 🧠"

# Search through your entries
cargo run search "rust" --since "last week"

# Export your data
cargo run export --format csv --output my-thoughts.csv

# View stats about your logging habits
cargo run stats --monthly
```

But honestly? The real product here is **knowledge**. Each feature is an excuse to dig deeper into how data systems work.

## Current Status 🛠️

Right now, this project is basically a fancy "Hello, world!" 😅 

But that's okay! Every journey starts with a single step, and I'm planning to build this incrementally:

1. **Phase 1**: Basic CLI structure and argument parsing
2. **Phase 2**: SQLite setup, schema design, and basic CRUD operations
3. **Phase 3**: Data modeling and serialization patterns
4. **Phase 4**: Search functionality and indexing strategies
5. **Phase 5**: Export/import features and data migration patterns
6. **Phase 6**: Performance optimization and advanced database concepts
7. **Phase 7**: TUI

## Getting Started

```bash
# Clone and build
git clone <your-repo-url>
cd personal_log
cargo build

# Run it (currently just says hello!)
cargo run

# Run tests (when I write some 😬)
cargo test

# Check your code style
cargo clippy
cargo fmt
```

## Learning Resources 📚

As I build this, I'm documenting interesting discoveries and resources:

- [SQLite Documentation](https://sqlite.org/docs.html) - The source of truth
- [Database Internals](https://www.databass.dev/) - Amazing book on how databases work
- [The Rust Programming Language](https://doc.rust-lang.org/book/) - For all things Rust
- [clap Documentation](https://docs.rs/clap/) - Building great CLIs

## Contributing to My Learning 🤝

This is primarily a personal learning project, but if you're on a similar journey or have insights to share, I'd love to hear from you! Feel free to:

- Share interesting articles or resources about database internals
- Suggest experiments or features that would be educational to implement
- Point out better ways to do things (I'm still learning!)
- Share your own database learning projects

## License

MIT - Feel free to fork this and start your own database learning adventure!

---

*"The best way to understand something is to build it yourself"* - Some smart person, probably 🤓
