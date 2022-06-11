import pandas as pd
import typer

import gradio as gr
import pandas as pd

from typing import List, Tuple
from xkcd_explorer import Client, Comic
from datasets import load_dataset

global CLIENT

app = typer.Typer()


def search(prompt: str) -> Tuple[List[str], pd.DataFrame]:
    """
    Given a text prompt, search for similar comics inside the XKCD database.

    Parameters
    ----------
    prompt: str

    Returns
    -------
    comic_urls, rows: Tuple[List[str], List[List[str]]]
    """
    global CLIENT
    results = CLIENT.search(prompt, topk=10)

    image_urls = []
    rows = []

    # Prepare outputs for Gradio
    for res in results:
        comic = res.comic
        image_urls.append(comic.image_url)
        rows.append(dict(title=comic.title, url=comic.url, l2_distance=res.distance))

    return image_urls, pd.DataFrame(rows)


def insert(example):
    """
    Insert a given comic into the XKCD database

    Parameters
    ----------
    example: a row of the XKCD dataset
    """
    global CLIENT
    CLIENT.insert(
        Comic(
            uuid=example["id"],
            title=example["title"],
            url=example["url"],
            image_url=example["image_url"],
            explained_url=example["explained_url"],
            text=example["text"],
        )
    )


@app.command()
def gradio(server_url: str = typer.Argument("localhost:50051", envvar="SERVER_URL")):
    """
    Launch Gradio app
    """
    global CLIENT
    CLIENT = Client(server_url)

    demo = gr.Interface(
        fn=search,
        inputs=[gr.Textbox("Machine Learning", label="Search prompt")],
        outputs=[gr.Gallery(label="Comics"), gr.DataFrame(label="Similar Comics")],
        title="XKCD Explorer",
        description="Given a prompt, find similar comics in the XKCD database.",
    )

    demo.launch()


@app.command()
def import_data(
    server_url: str = typer.Argument("localhost:50051", envvar="SERVER_URL")
):
    """
    Import XKCD dataset
    """
    global CLIENT
    CLIENT = Client(server_url)

    dataset = load_dataset("olivierdehaene/xkcd", split="train")

    # Concatenate title and transcript for a better embedding
    dataset = dataset.map(
        lambda example: {
            "text": str(example["title"]) + "\n" + str(example["transcript"])
        }
    )
    dataset.map(insert)


if __name__ == "__main__":
    app()
