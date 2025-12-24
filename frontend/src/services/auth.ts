import { api } from './api';
import type {
    LoginRequest,
    LoginResponse,
    RefreshTokenRequest,
    RefreshTokenResponse,
    MessageResponse,
} from '../types';

export const authService = {
    async login(credentials: LoginRequest): Promise<LoginResponse> {
        const response = await api.post<LoginResponse>('/auth/login', credentials);

        // Store tokens
        localStorage.setItem('access_token', response.data.access_token);
        localStorage.setItem('refresh_token', response.data.refresh_token);
        localStorage.setItem('user', JSON.stringify(response.data.user));

        return response.data;
    },

    async refreshToken(): Promise<RefreshTokenResponse> {
        const refreshToken = localStorage.getItem('refresh_token');
        if (!refreshToken) {
            throw new Error('No refresh token available');
        }

        const request: RefreshTokenRequest = { refresh_token: refreshToken };
        const response = await api.post<RefreshTokenResponse>('/auth/refresh', request);

        // Update access token
        localStorage.setItem('access_token', response.data.access_token);

        return response.data;
    },

    async logout(): Promise<void> {
        try {
            await api.post<MessageResponse>('/auth/logout');
        } finally {
            // Clear tokens regardless of API response
            localStorage.removeItem('access_token');
            localStorage.removeItem('refresh_token');
            localStorage.removeItem('user');
        }
    },

    getStoredUser() {
        const user = localStorage.getItem('user');
        return user ? JSON.parse(user) : null;
    },

    isAuthenticated(): boolean {
        return !!localStorage.getItem('access_token');
    },
};
