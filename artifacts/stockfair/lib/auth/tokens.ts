import AsyncStorage from '@react-native-async-storage/async-storage';

const ACCESS_TOKEN_KEY  = '@stockfair:access_token';
const REFRESH_TOKEN_KEY = '@stockfair:refresh_token';

export interface AuthTokens {
  accessToken: string;
  refreshToken: string;
}

export async function saveTokens(tokens: AuthTokens): Promise<void> {
  await AsyncStorage.multiSet([
    [ACCESS_TOKEN_KEY,  tokens.accessToken],
    [REFRESH_TOKEN_KEY, tokens.refreshToken],
  ]);
}

export async function getTokens(): Promise<AuthTokens | null> {
  const pairs = await AsyncStorage.multiGet([ACCESS_TOKEN_KEY, REFRESH_TOKEN_KEY]);
  const accessToken  = pairs[0][1];
  const refreshToken = pairs[1][1];
  if (!accessToken || !refreshToken) return null;
  return { accessToken, refreshToken };
}

export async function clearTokens(): Promise<void> {
  await AsyncStorage.multiRemove([ACCESS_TOKEN_KEY, REFRESH_TOKEN_KEY]);
}

export async function getAccessToken(): Promise<string | null> {
  return AsyncStorage.getItem(ACCESS_TOKEN_KEY);
}
