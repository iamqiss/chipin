// Mirrors src/models/user.rs on the motherlode side

export type SupportedLanguage =
  | 'en' | 'zu' | 'xh' | 'af'
  | 'nso' | 'tn' | 'st' | 'ts'
  | 'ss' | 've' | 'nr';

export type AppTheme = 'obsidian' | 'forge' | 'bloom';

export type Gender =
  | 'male' | 'female'
  | 'non_binary' | 'prefer_not_to_say';

export type StokvelInterest =
  | 'rotation' | 'burial'
  | 'investment' | 'grocery' | 'social';

export type OtpPurpose =
  | 'register' | 'reset_password' | 'withdraw';

// ── User ──────────────────────────────────────────────────────────────────────

export interface UserProfile {
  id: string;
  phone: string;
  email: string | null;
  full_name: string;
  avatar_url: string | null;
  language: SupportedLanguage;
  theme: AppTheme;
  is_kyc_verified: boolean;
  created_at: string;
}

// ── Registration steps ────────────────────────────────────────────────────────

export interface RegisterStep1Payload {
  first_name: string;
  last_name: string;
  date_of_birth: string; // DD/MM/YYYY
  gender: Gender;
}

export interface RegisterStep2Payload {
  session_key: string;
  phone: string;
  email?: string;
  password: string;
  confirm_password: string;
}

export interface RegisterStep3Payload {
  phone: string;
  language: SupportedLanguage;
}

export interface RegisterStep4Payload {
  phone: string;
  interests: StokvelInterest[];
  theme?: AppTheme;
  terms_accepted: boolean;
}

// ── OTP ───────────────────────────────────────────────────────────────────────

export interface SendOtpPayload {
  phone: string;
  purpose: OtpPurpose;
}

export interface VerifyOtpPayload {
  phone: string;
  purpose: OtpPurpose;
  code: string;
}

// ── Sign in ───────────────────────────────────────────────────────────────────

export interface SignInPayload {
  identifier: string; // phone or email
  password: string;
}

// ── Password reset ────────────────────────────────────────────────────────────

export interface ForgotPasswordPayload {
  phone: string;
}

export interface ResetPasswordPayload {
  phone: string;
  otp_token: string;
  new_password: string;
  confirm_new_password: string;
}

// ── Responses ─────────────────────────────────────────────────────────────────

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  user: UserProfile;
}

export interface OtpSentResponse {
  message: string;
  expires_in_seconds: number;
  debug_otp?: string; // only in development
}

export interface OtpVerifiedResponse {
  otp_token: string;
  message: string;
}

export interface MessageResponse {
  message: string;
}

export interface Step1Response {
  session_key: string;
  message: string;
}

export interface Step2Response {
  phone: string;
  message: string;
}
