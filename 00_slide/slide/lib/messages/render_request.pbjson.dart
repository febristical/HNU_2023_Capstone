//
//  Generated code. Do not modify.
//  source: render_request.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use renderRequestDescriptor instead')
const RenderRequest$json = {
  '1': 'RenderRequest',
  '2': [
    {'1': 'angle', '3': 1, '4': 1, '5': 1, '10': 'angle'},
    {'1': 'style', '3': 2, '4': 1, '5': 13, '10': 'style'},
  ],
};

/// Descriptor for `RenderRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List renderRequestDescriptor = $convert.base64Decode(
    'Cg1SZW5kZXJSZXF1ZXN0EhQKBWFuZ2xlGAEgASgBUgVhbmdsZRIUCgVzdHlsZRgCIAEoDVIFc3'
    'R5bGU=');

@$core.Deprecated('Use renderResponseDescriptor instead')
const RenderResponse$json = {
  '1': 'RenderResponse',
  '2': [
    {'1': 'image', '3': 1, '4': 1, '5': 12, '10': 'image'},
  ],
};

/// Descriptor for `RenderResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List renderResponseDescriptor = $convert.base64Decode(
    'Cg5SZW5kZXJSZXNwb25zZRIUCgVpbWFnZRgBIAEoDFIFaW1hZ2U=');

