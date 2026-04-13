import { api } from '@/lib/api/client';
import { ENDPOINTS } from '@/constants/api';
import type {
  AuthResponse,
  ForgotPasswordPayload,
  MessageResponse,
  OtpSentResponse,
  OtpVerifiedResponse,
  RegisterStep1Payload,
  RegisterStep2Payload,
  RegisterStep3Payload,
  RegisterStep4Payload,
  ResetPasswordPayload,
  SendOtpPayload,
  SignInPayload,
  Step1Response,
  Step2Response,
  VerifyOtpPayload,
} from './types';

// ── Registration ──────────────────────────────────────────────────────────────

export const registerStep1 = (payload: RegisterStep1Payload) =>
  api.public.post<Step1Response>(ENDPOINTS.AUTH_REGISTER_STEP1, payload);

export const registerStep2 = (payload: RegisterStep2Payload) =>
  api.public.post<Step2Response>(ENDPOINTS.AUTH_REGISTER_STEP2, payload);

export const registerStep3 = (payload: RegisterStep3Payload) =>
  api.public.post<MessageResponse>(ENDPOINTS.AUTH_REGISTER_STEP3, payload);

export const registerStep4 = (payload: RegisterStep4Payload) =>
  api.public.post<AuthResponse>(ENDPOINTS.AUTH_REGISTER_STEP4, payload);

// ── OTP ───────────────────────────────────────────────────────────────────────

export const sendOtp = (payload: SendOtpPayload) =>
  api.public.post<OtpSentResponse>(ENDPOINTS.AUTH_OTP_SEND, payload);

export const verifyOtp = (payload: VerifyOtpPayload) =>
  api.public.post<OtpVerifiedResponse>(ENDPOINTS.AUTH_OTP_VERIFY, payload);

// ── Sign in / out ─────────────────────────────────────────────────────────────

export const signIn = (payload: SignInPayload) =>
  api.public.post<AuthResponse>(ENDPOINTS.AUTH_SIGNIN, payload);

export const signOut = (refreshToken: string) =>
  api.post<MessageResponse>(ENDPOINTS.AUTH_SIGNOUT, { refresh_token: refreshToken });

// ── Password reset ────────────────────────────────────────────────────────────

export const forgotPassword = (payload: ForgotPasswordPayload) =>
  api.public.post<OtpSentResponse>(ENDPOINTS.AUTH_FORGOT_PASSWORD, payload);

export const resetPassword = (payload: ResetPasswordPayload) =>
  api.public.post<MessageResponse>(ENDPOINTS.AUTH_RESET_PASSWORD, payload);

// ── Current user ──────────────────────────────────────────────────────────────

export const getMe = () =>
  api.get<{ user: import('./types').UserProfile }>(ENDPOINTS.AUTH_ME);
