<h2 align="center">
  oi
</h2>

oi is a concept for a command line wrapper for calling out to large-language
models.

Right now, all oi does is let you ask a question to the OpenAI gpt-3.5-turbo
model with a built in prompt. If I like this idea I'm planning on making the
prompt configurable and also make the LLM configurable, to where you can have it
call out to different APIs or even use an LLM like llama.cpp running on your
machine.

PRs welcome.

## Installation

Pull down the repository and ensure you have cargo and rust installed.

On linux:

Then:

```
cargo build --release
sudo mv ./target/release/oi /usr/local/bin/
```

## Usage

```
> oi "how do I print out all files in this directory sorted by modified time?"
You can list all files in the current directory ordered by modification time using the command "ls -lt".
```

## License

MIT
