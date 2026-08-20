# Contributing

Issues and pull requests are more than welcome.

### dev install

This project uses [uv](https://docs.astral.sh/uv/) to manage its development environment.

```bash
git clone https://github.com/vincentsarago/color-operations.git
cd color-operations
uv sync
```

You can then run the tests with the following command:

```sh
uv run pytest --cov color_operations --cov-report term-missing
```

### pre-commit

This repo is set to use `pre-commit` to run *isort*, *flake8*, *pydocstring*, *black* ("uncompromising Python code formatter") and mypy when committing new code.

```bash
uv run pre-commit install
```
