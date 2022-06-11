import typer

from pathlib import Path
from transformers import AutoTokenizer, AutoModel


app = typer.Typer()


@app.command()
def download(
    model_ckpt: str = "sentence-transformers/multi-qa-mpnet-base-dot-v1",
    model_root_dir: Path = "models",
):
    model_root_dir.mkdir(parents=True, exist_ok=True)

    tokenizer = AutoTokenizer.from_pretrained(model_ckpt)
    model = AutoModel.from_pretrained(model_ckpt)

    tokenizer.save_pretrained(model_root_dir / model_ckpt)
    model.save_pretrained(model_root_dir / model_ckpt)


if __name__ == "__main__":
    app()
