import os

import gradio as gr

from typing import List
from xkcd_explorer import Client

XKCD_EXPLORER_SERVER_URL = os.environ.get("XKCD_EXPLORER_SERVER_URL", "localhost:50051")
CLIENT = Client(XKCD_EXPLORER_SERVER_URL)


def search(prompt: str, topk: int) -> List[str]:
    """
    Given a text prompt, search for similar comics inside the XKCD database.

    Parameters
    ----------
    prompt: str
    topk: int

    Returns
    -------
    comic_urls: List[str]
    """
    results = CLIENT.search(prompt, topk)
    return [res.comic.image_url for res in results]


def app():
    """
    Launch Gradio app
    """
    demo = gr.Interface(
        fn=search,
        inputs=[
            gr.Textbox("Machine Learning", label="Search prompt"),
            gr.Slider(0, 100, 10, label="Maximum number of comics to return"),
        ],
        outputs=gr.Gallery(label="Similar Comics"),
        title="XKCD Explorer",
        description="Given a prompt, find similar comics in the XKCD database.",
        examples=[
            # [2, cheetah, 12, 12, 4, 4],
        ],
    )

    demo.launch()


if __name__ == "__main__":
    app()
