from dataclasses import dataclass
from xkcd_explorer.pb.xkcd_explorer_pb2 import Object


@dataclass
class Comic:
    """Comic dataclass"""

    uuid: int
    title: str
    url: str
    image_url: str
    explained_url: str

    def to_request(self) -> Object.Comic:
        return Object.Comic(
            id=self.uuid,
            title=self.title,
            url=self.url,
            image_url=self.image_url,
            explained_url=self.explained_url,
        )

    @classmethod
    def from_response(cls, res: Object.Comic) -> "Comic":
        return cls(
            uuid=res.id,
            title=res.title,
            url=res.url,
            image_url=res.image_url,
            explained_url=res.explained_url,
        )


@dataclass
class Result:
    """Search result dataclass"""

    comic: Comic
    similarity: float

    @classmethod
    def from_response(cls, res: Object.Similarity) -> "Result":
        return cls(comic=Comic.from_response(res.comic), similarity=res.similarity)
