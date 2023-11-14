//
//  Generated code. Do not modify.
//  source: render_request.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class RenderRequest extends $pb.GeneratedMessage {
  factory RenderRequest({
    $core.double? angle,
    $core.int? style,
  }) {
    final $result = create();
    if (angle != null) {
      $result.angle = angle;
    }
    if (style != null) {
      $result.style = style;
    }
    return $result;
  }
  RenderRequest._() : super();
  factory RenderRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RenderRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RenderRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'render_request'), createEmptyInstance: create)
    ..a<$core.double>(1, _omitFieldNames ? '' : 'angle', $pb.PbFieldType.OD)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'style', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RenderRequest clone() => RenderRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RenderRequest copyWith(void Function(RenderRequest) updates) => super.copyWith((message) => updates(message as RenderRequest)) as RenderRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RenderRequest create() => RenderRequest._();
  RenderRequest createEmptyInstance() => create();
  static $pb.PbList<RenderRequest> createRepeated() => $pb.PbList<RenderRequest>();
  @$core.pragma('dart2js:noInline')
  static RenderRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RenderRequest>(create);
  static RenderRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.double get angle => $_getN(0);
  @$pb.TagNumber(1)
  set angle($core.double v) { $_setDouble(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAngle() => $_has(0);
  @$pb.TagNumber(1)
  void clearAngle() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get style => $_getIZ(1);
  @$pb.TagNumber(2)
  set style($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasStyle() => $_has(1);
  @$pb.TagNumber(2)
  void clearStyle() => clearField(2);
}

class RenderResponse extends $pb.GeneratedMessage {
  factory RenderResponse({
    $core.List<$core.int>? image,
  }) {
    final $result = create();
    if (image != null) {
      $result.image = image;
    }
    return $result;
  }
  RenderResponse._() : super();
  factory RenderResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RenderResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RenderResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'render_request'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'image', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RenderResponse clone() => RenderResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RenderResponse copyWith(void Function(RenderResponse) updates) => super.copyWith((message) => updates(message as RenderResponse)) as RenderResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RenderResponse create() => RenderResponse._();
  RenderResponse createEmptyInstance() => create();
  static $pb.PbList<RenderResponse> createRepeated() => $pb.PbList<RenderResponse>();
  @$core.pragma('dart2js:noInline')
  static RenderResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RenderResponse>(create);
  static RenderResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get image => $_getN(0);
  @$pb.TagNumber(1)
  set image($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasImage() => $_has(0);
  @$pb.TagNumber(1)
  void clearImage() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');

const ID = 4;