Cow Facts
=========

Moo along! Nothing to see here!

This program is a reimagining of the program [Cowsay](https://en.wikipedia.org/wiki/Cowsay) and is a toy example and learning ground for interacting with AI tooling and with [OpenAI GPT API](https://platform.openai.com/docs/overview).


USAGE
-----

Build `cowfact` using cargo:
```bash
cargo build --release
```

Execute `cowfact`:
```bash
OPENAI_API_KEY=<YOUR_API_KEY> ./target/release/cowfact
```

Example output:

```
 ( I've got an awe-inspiring cow fact for you - did you know that cows have special )
 ( valves in their ears that help regulate their heart rate when they're listening  )
 ( to music, a phenomenon that's been studied in music therapy and has even been    )
 ( featured in a song by the American rock band, Pearl Jam?                         )
    \   ^__^
     \  (oo)\_______
        (__)\       )\/\
            ||----w |
            ||     ||
```