export type ApiErrorI18nKeys = Readonly<{
  title: string;
  description?: string;
}>;

function message(title: string, description?: string): ApiErrorI18nKeys {
  return { title, description };
}

/** Maps stable backend error codes to domain-owned toast copy. */
export const API_ERROR_I18N_KEYS: Readonly<Record<string, ApiErrorI18nKeys>> = {
  // Shared HTTP and infrastructure errors.
  bad_request: message(
    "common:errors.bad_request.title",
    "common:errors.bad_request.description"
  ),
  unauthorized: message(
    "common:errors.unauthorized.title",
    "common:errors.unauthorized.description"
  ),
  forbidden: message("common:errors.forbidden"),
  not_found: message("common:errors.not_found"),
  internal_server_error: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  conflict: message(
    "common:errors.conflict.title",
    "common:errors.conflict.description"
  ),
  locked: message(
    "common:errors.locked.title",
    "common:errors.locked.description"
  ),
  too_many_requests: message(
    "common:errors.too_many_requests.title",
    "common:errors.too_many_requests.description"
  ),
  unprocessable_entity: message(
    "common:errors.unprocessable_entity.title",
    "common:errors.unprocessable_entity.description"
  ),
  invalid_json: message("common:errors.invalid_json"),
  validation_failed: message("common:errors.validation_failed"),
  invalid_path: message("common:errors.invalid_path"),
  path_parameters_missing: message("common:errors.path_parameters_missing"),
  path_extraction_failed: message("common:errors.path_extraction_failed"),
  invalid_query: message("common:errors.invalid_query"),
  query_extraction_failed: message("common:errors.query_extraction_failed"),
  extension_missing: message("common:errors.extension_missing"),
  extension_extraction_failed: message(
    "common:errors.extension_extraction_failed"
  ),
  rate_limit_exceeded: message(
    "common:errors.rate_limit_exceeded.title",
    "common:errors.rate_limit_exceeded.description"
  ),
  rate_limit_key_unavailable: message(
    "common:errors.rate_limit_key_unavailable"
  ),
  rate_limit_failed: message(
    "common:errors.rate_limit_failed.title",
    "common:errors.rate_limit_failed.description"
  ),
  tower_sessions: message(
    "account:errors.session_error.title",
    "account:errors.session_error.description"
  ),
  http: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  multipart: message("common:errors.multipart"),
  db: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  cache: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  env: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  event: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  captcha: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  idp: message(
    "account:errors.idp_request_failed.title",
    "account:errors.idp_request_failed.description"
  ),
  media: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  queue: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  cluster: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  other: message(
    "common:errors.internal_server_error.title",
    "common:errors.internal_server_error.description"
  ),
  host_extract_failed: message("common:errors.host_extract_failed"),
  ip_extract_failed: message("common:errors.ip_extract_failed"),
  invalid: message(
    "common:errors.invalid.title",
    "common:errors.invalid.description"
  ),
  avatar_not_found: message("common:errors.media.avatar_not_found"),
  dont_need_generate_captcha: message("common:errors.captcha.not_required"),
  forbidden_file_type: message("common:errors.media.forbidden_file_type"),
  invalid_mime_type: message("common:errors.media.invalid_mime_type"),
  missing_content_type: message("common:errors.media.missing_content_type"),
  no_file: message(
    "common:errors.media.no_file.title",
    "common:errors.media.no_file.description"
  ),
  size_too_large: message("common:errors.media.size_too_large"),
  upload_media_10m: message("common:errors.media.upload_limit_10m"),
  upload_media_24h: message("common:errors.media.upload_limit_24h"),

  // Account and identity errors.
  captcha_invalid: message(
    "account:errors.captcha_invalid.title",
    "account:errors.captcha_invalid.description"
  ),
  email_already_exists: message("account:errors.email_already_exists"),
  email_already_verified: message("account:errors.email_already_verified"),
  email_code_expired: message(
    "account:errors.email_code_expired.title",
    "account:errors.email_code_expired.description"
  ),
  email_code_incorrect: message("account:errors.email_code_incorrect"),
  email_disabled: message("account:errors.email_disabled"),
  email_not_found: message("account:errors.email_not_found"),
  email_send_too_frequently: message(
    "account:errors.email_send_too_frequently.title",
    "account:errors.email_send_too_frequently.description"
  ),
  idp_already_bound: message("account:errors.idp_already_bound"),
  idp_not_found: message("account:errors.idp_not_found"),
  idp_pending_mismatch: message(
    "account:errors.idp_pending_mismatch.title",
    "account:errors.idp_pending_mismatch.description"
  ),
  idp_script_invalid: message("account:errors.idp_script_invalid"),
  idp_registration_disabled: message(
    "account:errors.idp_registration_disabled"
  ),
  registration_idp_cannot_be_unbound: message(
    "account:errors.registration_idp_cannot_be_unbound.title",
    "account:errors.registration_idp_cannot_be_unbound.description"
  ),
  invalid_or_expired_token: message("account:errors.invalid_or_expired_token"),
  password_invalid: message("account:errors.password_invalid"),
  registration_conflict: message(
    "account:errors.registration_conflict.title",
    "account:errors.registration_conflict.description"
  ),
  registration_disabled: message("account:errors.registration_disabled"),
  session_error: message(
    "account:errors.session_error.title",
    "account:errors.session_error.description"
  ),
  user_idp_already_bound: message("account:errors.user_idp_already_bound"),
  user_idp_not_found: message("account:errors.user_idp_not_found"),
  user_not_found: message("account:errors.user_not_found"),
  username_already_exists: message("account:errors.username_already_exists"),

  // Competition and challenge errors.
  game_blacked_out: message("game:errors.blacked_out"),
  game_is_not_ongoing: message("game:errors.not_ongoing"),
  game_not_found: message("game:errors.not_found"),
  game_not_ongoing: message("game:errors.not_ongoing"),
  game_paused: message("game:errors.paused"),
  icon_not_found: message("game:errors.icon_not_found"),
  poster_not_found: message("game:errors.poster_not_found"),
  challenge_already_in_game: message("challenge:errors.already_in_game"),
  challenge_has_not_attachment: message(
    "challenge:errors.attachment_not_found"
  ),
  challenge_not_found: message("challenge:errors.not_found"),
  game_challenge_not_found: message(
    "challenge:errors.game_challenge_not_found"
  ),
  either_user_or_team: message(
    "challenge:errors.user_or_team_required.title",
    "challenge:errors.user_or_team_required.description"
  ),
  checker_key_generation_failed: message(
    "challenge:errors.checker_key_generation_failed.title",
    "challenge:errors.checker_key_generation_failed.description"
  ),
  checker_key_already_exists: message(
    "challenge:errors.checker_key_already_exists.title",
    "challenge:errors.checker_key_already_exists.description"
  ),

  // Team errors.
  invalid_invite_token: message("team:errors.invalid_invite_token"),
  invalid_team: message("team:errors.invalid_team"),
  member_limit_not_satisfied: message(
    "team:errors.member_limit_not_satisfied.title",
    "team:errors.member_limit_not_satisfied.description"
  ),
  no_invite_token: message(
    "team:errors.no_invite_token.title",
    "team:errors.no_invite_token.description"
  ),
  team_has_no_other_member: message("team:errors.no_other_member"),
  team_not_found: message("team:errors.not_found"),
  team_not_preparing: message("team:errors.not_preparing"),
  user_already_in_game: message("team:errors.user_already_in_game"),

  // Instance and submission errors.
  challenge_instance_invalid: message(
    "instance:errors.challenge_invalid.title",
    "instance:errors.challenge_invalid.description"
  ),
  no_more_renewal: message("instance:errors.no_more_renewal"),
  renewal_within_10_minutes: message(
    "instance:errors.renewal_within_10_minutes"
  ),
  too_many_team_pods: message("instance:errors.too_many_team_instances"),
  too_many_user_pods: message("instance:errors.too_many_user_instances"),
  cheated: message(
    "submission:errors.cheated.title",
    "submission:errors.cheated.description"
  ),
  correct_submission_already_exists: message(
    "submission:errors.correct_submission_already_exists.title",
    "submission:errors.correct_submission_already_exists.description"
  ),
  submission: message(
    "submission:errors.too_frequent.title",
    "submission:errors.too_frequent.description"
  ),
};

export const API_ERROR_FALLBACK_KEYS = message(
  "common:errors.default.title",
  "common:errors.default.description"
);

export const NETWORK_ERROR_I18N_KEYS = message(
  "common:errors.network.title",
  "common:errors.network.description"
);

export const TIMEOUT_ERROR_I18N_KEYS = message(
  "common:errors.timeout.title",
  "common:errors.timeout.description"
);

export const SERVICE_UNAVAILABLE_I18N_KEYS = message(
  "common:errors.service_unavailable.title",
  "common:errors.service_unavailable.description"
);
