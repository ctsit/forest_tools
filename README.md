# Forest Tools

Project to export triples from a Jena MySQL SDB database. This tool can export a 37 million row Quads table in around 10 minutes. The program prints the N-Quads to stdout.

## Running

The program expects an environment variable named `MYSQL_URL` containing the URL connection string.

```bash
    $ MYSQL_URL=mysql://<username>:<password>@<host>/<dbname> ./forest_tools >content.nq
```

## Building

```bash
    $ cargo build --release
```

## Building for Linux on MacOS

I used [muslrust](https://github.com/clux/muslrust) to cross-compile the program to linux.

```bash
    $ docker pull clux/muslrust
    $ docker run -v $PWD:/volume --rm -t clux/muslrust cargo build
```

## LICENSE

   Copyright 2019 Hunter Jarrell

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
