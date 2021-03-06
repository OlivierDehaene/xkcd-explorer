# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from . import xkcd_explorer_pb2 as xkcd__explorer__pb2


class XkcdExplorerStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Exists = channel.unary_unary(
                '/xkcd_explorer.v1.XkcdExplorer/Exists',
                request_serializer=xkcd__explorer__pb2.Object.ComicRequest.SerializeToString,
                response_deserializer=xkcd__explorer__pb2.ExistsResponse.FromString,
                )
        self.Get = channel.unary_unary(
                '/xkcd_explorer.v1.XkcdExplorer/Get',
                request_serializer=xkcd__explorer__pb2.Object.ComicRequest.SerializeToString,
                response_deserializer=xkcd__explorer__pb2.Object.Comic.FromString,
                )
        self.Search = channel.unary_unary(
                '/xkcd_explorer.v1.XkcdExplorer/Search',
                request_serializer=xkcd__explorer__pb2.Search.Request.SerializeToString,
                response_deserializer=xkcd__explorer__pb2.Search.Response.FromString,
                )
        self.Insert = channel.unary_unary(
                '/xkcd_explorer.v1.XkcdExplorer/Insert',
                request_serializer=xkcd__explorer__pb2.Object.Comic.SerializeToString,
                response_deserializer=xkcd__explorer__pb2.Empty.FromString,
                )


class XkcdExplorerServicer(object):
    """Missing associated documentation comment in .proto file."""

    def Exists(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Get(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Search(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Insert(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_XkcdExplorerServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'Exists': grpc.unary_unary_rpc_method_handler(
                    servicer.Exists,
                    request_deserializer=xkcd__explorer__pb2.Object.ComicRequest.FromString,
                    response_serializer=xkcd__explorer__pb2.ExistsResponse.SerializeToString,
            ),
            'Get': grpc.unary_unary_rpc_method_handler(
                    servicer.Get,
                    request_deserializer=xkcd__explorer__pb2.Object.ComicRequest.FromString,
                    response_serializer=xkcd__explorer__pb2.Object.Comic.SerializeToString,
            ),
            'Search': grpc.unary_unary_rpc_method_handler(
                    servicer.Search,
                    request_deserializer=xkcd__explorer__pb2.Search.Request.FromString,
                    response_serializer=xkcd__explorer__pb2.Search.Response.SerializeToString,
            ),
            'Insert': grpc.unary_unary_rpc_method_handler(
                    servicer.Insert,
                    request_deserializer=xkcd__explorer__pb2.Object.Comic.FromString,
                    response_serializer=xkcd__explorer__pb2.Empty.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'xkcd_explorer.v1.XkcdExplorer', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class XkcdExplorer(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def Exists(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/xkcd_explorer.v1.XkcdExplorer/Exists',
            xkcd__explorer__pb2.Object.ComicRequest.SerializeToString,
            xkcd__explorer__pb2.ExistsResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Get(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/xkcd_explorer.v1.XkcdExplorer/Get',
            xkcd__explorer__pb2.Object.ComicRequest.SerializeToString,
            xkcd__explorer__pb2.Object.Comic.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Search(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/xkcd_explorer.v1.XkcdExplorer/Search',
            xkcd__explorer__pb2.Search.Request.SerializeToString,
            xkcd__explorer__pb2.Search.Response.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Insert(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/xkcd_explorer.v1.XkcdExplorer/Insert',
            xkcd__explorer__pb2.Object.Comic.SerializeToString,
            xkcd__explorer__pb2.Empty.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
