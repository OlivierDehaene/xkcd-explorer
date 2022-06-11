import grpc

from typing import List

from xkcd_explorer.pb import xkcd_explorer_pb2, xkcd_explorer_pb2_grpc
from xkcd_explorer.types import Comic, Result


class Client:
    """
    XKCD Explorer client

    Args:
        url (str): xkcd-explorer-server gRPC url. Example: xkcd-explorer-server:50051
    """

    def __init__(self, url: str):
        self.url = url
        self.channel = grpc.insecure_channel(self.url)
        self.stub = xkcd_explorer_pb2_grpc.XkcdExplorerStub(self.channel)

    def exists(self, uuid: str) -> bool:
        """
        Test if the provided uuid exists in the XKCD Explorer database.

        Args:
            uuid (str): comic id

        Returns:
            bool: true if exists, false otherwise
        """
        res = self.stub.Exists(xkcd_explorer_pb2.Object.ComicRequest(id=uuid))
        return res.exists

    def get(self, uuid: str) -> Comic:
        """
        Get the comic associated to a specific uuid in the XKCD Explorer database

        Args:
            uuid (str): comic id

        Returns:
            Comic, the comic information

        Raises:
            RPCError: if the uuid does not exist
        """
        res = self.stub.Get(xkcd_explorer_pb2.Object.ComicRequest(id=uuid))
        return Comic.from_response(res)

    def insert(self, comic: Comic):
        """
        Insert a Comic dataclass in the XKCD Explorer database

        Args:
            comic (Comic): Comic dataclass

        Raises:
            RPCError: if an entry with the same uuid already exist
        """
        self.stub.Insert(comic.to_request())

    def search(
        self,
        prompt: str,
        topk: int = 10,
    ) -> List[Result]:
        """
        Asymmetric semantic search through the database given the prompt

        Args:
            prompt (str): Prompt to encode and search for in the database
            topk (int): number of results

        Returns:
            List[Result]: List of similar entries
        """
        res = self.stub.Search(
            xkcd_explorer_pb2.Search.Request(prompt=prompt, topk=topk)
        )
        return [Result.from_response(r) for r in res.results]
