# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: xkcd_explorer.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x13xkcd_explorer.proto\x12\x10xkcd_explorer.v1\" \n\x0e\x45xistsResponse\x12\x0e\n\x06\x65xists\x18\x01 \x01(\x08\"\x81\x01\n\x06Search\x1a\x35\n\x07Request\x12\x0e\n\x06prompt\x18\x01 \x01(\t\x12\x11\n\x04topk\x18\x03 \x01(\rH\x00\x88\x01\x01\x42\x07\n\x05_topk\x1a@\n\x08Response\x12\x34\n\x07results\x18\x01 \x03(\x0b\x32#.xkcd_explorer.v1.Object.Similarity\"\xd0\x01\n\x06Object\x1aY\n\x05\x43omic\x12\n\n\x02id\x18\x01 \x01(\x05\x12\r\n\x05title\x18\x02 \x01(\t\x12\x0b\n\x03url\x18\x03 \x01(\t\x12\x11\n\timage_url\x18\x04 \x01(\t\x12\x15\n\rexplained_url\x18\x05 \x01(\t\x1a\x1a\n\x0c\x43omicRequest\x12\n\n\x02id\x18\x01 \x01(\x05\x1aO\n\nSimilarity\x12-\n\x05\x63omic\x18\x01 \x01(\x0b\x32\x1e.xkcd_explorer.v1.Object.Comic\x12\x12\n\nsimilarity\x18\x02 \x01(\x02\"\x07\n\x05\x45mpty2\xd0\x02\n\x0cXkcdExplorer\x12V\n\x06\x45xists\x12%.xkcd_explorer.v1.Object.ComicRequest\x1a .xkcd_explorer.v1.ExistsResponse\"\x03\x90\x02\x02\x12Q\n\x03Get\x12%.xkcd_explorer.v1.Object.ComicRequest\x1a\x1e.xkcd_explorer.v1.Object.Comic\"\x03\x90\x02\x02\x12R\n\x06Search\x12 .xkcd_explorer.v1.Search.Request\x1a!.xkcd_explorer.v1.Search.Response\"\x03\x90\x02\x02\x12\x41\n\x06Insert\x12\x1e.xkcd_explorer.v1.Object.Comic\x1a\x17.xkcd_explorer.v1.Emptyb\x06proto3')



_EXISTSRESPONSE = DESCRIPTOR.message_types_by_name['ExistsResponse']
_SEARCH = DESCRIPTOR.message_types_by_name['Search']
_SEARCH_REQUEST = _SEARCH.nested_types_by_name['Request']
_SEARCH_RESPONSE = _SEARCH.nested_types_by_name['Response']
_OBJECT = DESCRIPTOR.message_types_by_name['Object']
_OBJECT_COMIC = _OBJECT.nested_types_by_name['Comic']
_OBJECT_COMICREQUEST = _OBJECT.nested_types_by_name['ComicRequest']
_OBJECT_SIMILARITY = _OBJECT.nested_types_by_name['Similarity']
_EMPTY = DESCRIPTOR.message_types_by_name['Empty']
ExistsResponse = _reflection.GeneratedProtocolMessageType('ExistsResponse', (_message.Message,), {
  'DESCRIPTOR' : _EXISTSRESPONSE,
  '__module__' : 'xkcd_explorer_pb2'
  # @@protoc_insertion_point(class_scope:xkcd_explorer.v1.ExistsResponse)
  })
_sym_db.RegisterMessage(ExistsResponse)

Search = _reflection.GeneratedProtocolMessageType('Search', (_message.Message,), {

  'Request' : _reflection.GeneratedProtocolMessageType('Request', (_message.Message,), {
    'DESCRIPTOR' : _SEARCH_REQUEST,
    '__module__' : 'xkcd_explorer_pb2'
    # @@protoc_insertion_point(class_scope:xkcd_explorer.v1.Search.Request)
    })
  ,

  'Response' : _reflection.GeneratedProtocolMessageType('Response', (_message.Message,), {
    'DESCRIPTOR' : _SEARCH_RESPONSE,
    '__module__' : 'xkcd_explorer_pb2'
    # @@protoc_insertion_point(class_scope:xkcd_explorer.v1.Search.Response)
    })
  ,
  'DESCRIPTOR' : _SEARCH,
  '__module__' : 'xkcd_explorer_pb2'
  # @@protoc_insertion_point(class_scope:xkcd_explorer.v1.Search)
  })
_sym_db.RegisterMessage(Search)
_sym_db.RegisterMessage(Search.Request)
_sym_db.RegisterMessage(Search.Response)

Object = _reflection.GeneratedProtocolMessageType('Object', (_message.Message,), {

  'Comic' : _reflection.GeneratedProtocolMessageType('Comic', (_message.Message,), {
    'DESCRIPTOR' : _OBJECT_COMIC,
    '__module__' : 'xkcd_explorer_pb2'
    # @@protoc_insertion_point(class_scope:xkcd_explorer.v1.Object.Comic)
    })
  ,

  'ComicRequest' : _reflection.GeneratedProtocolMessageType('ComicRequest', (_message.Message,), {
    'DESCRIPTOR' : _OBJECT_COMICREQUEST,
    '__module__' : 'xkcd_explorer_pb2'
    # @@protoc_insertion_point(class_scope:xkcd_explorer.v1.Object.ComicRequest)
    })
  ,

  'Similarity' : _reflection.GeneratedProtocolMessageType('Similarity', (_message.Message,), {
    'DESCRIPTOR' : _OBJECT_SIMILARITY,
    '__module__' : 'xkcd_explorer_pb2'
    # @@protoc_insertion_point(class_scope:xkcd_explorer.v1.Object.Similarity)
    })
  ,
  'DESCRIPTOR' : _OBJECT,
  '__module__' : 'xkcd_explorer_pb2'
  # @@protoc_insertion_point(class_scope:xkcd_explorer.v1.Object)
  })
_sym_db.RegisterMessage(Object)
_sym_db.RegisterMessage(Object.Comic)
_sym_db.RegisterMessage(Object.ComicRequest)
_sym_db.RegisterMessage(Object.Similarity)

Empty = _reflection.GeneratedProtocolMessageType('Empty', (_message.Message,), {
  'DESCRIPTOR' : _EMPTY,
  '__module__' : 'xkcd_explorer_pb2'
  # @@protoc_insertion_point(class_scope:xkcd_explorer.v1.Empty)
  })
_sym_db.RegisterMessage(Empty)

_XKCDEXPLORER = DESCRIPTOR.services_by_name['XkcdExplorer']
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _XKCDEXPLORER.methods_by_name['Exists']._options = None
  _XKCDEXPLORER.methods_by_name['Exists']._serialized_options = b'\220\002\002'
  _XKCDEXPLORER.methods_by_name['Get']._options = None
  _XKCDEXPLORER.methods_by_name['Get']._serialized_options = b'\220\002\002'
  _XKCDEXPLORER.methods_by_name['Search']._options = None
  _XKCDEXPLORER.methods_by_name['Search']._serialized_options = b'\220\002\002'
  _EXISTSRESPONSE._serialized_start=41
  _EXISTSRESPONSE._serialized_end=73
  _SEARCH._serialized_start=76
  _SEARCH._serialized_end=205
  _SEARCH_REQUEST._serialized_start=86
  _SEARCH_REQUEST._serialized_end=139
  _SEARCH_RESPONSE._serialized_start=141
  _SEARCH_RESPONSE._serialized_end=205
  _OBJECT._serialized_start=208
  _OBJECT._serialized_end=416
  _OBJECT_COMIC._serialized_start=218
  _OBJECT_COMIC._serialized_end=307
  _OBJECT_COMICREQUEST._serialized_start=309
  _OBJECT_COMICREQUEST._serialized_end=335
  _OBJECT_SIMILARITY._serialized_start=337
  _OBJECT_SIMILARITY._serialized_end=416
  _EMPTY._serialized_start=418
  _EMPTY._serialized_end=425
  _XKCDEXPLORER._serialized_start=428
  _XKCDEXPLORER._serialized_end=764
# @@protoc_insertion_point(module_scope)
