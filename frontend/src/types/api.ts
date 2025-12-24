// API types matching backend models

// Auth
export interface LoginRequest {
    email: string;
    password: string;
}

export interface LoginResponse {
    access_token: string;
    refresh_token: string;
    token_type: string;
    expires_in: number;
    user: UserWithRole;
}

export interface RefreshTokenRequest {
    refresh_token: string;
}

export interface RefreshTokenResponse {
    access_token: string;
    token_type: string;
    expires_in: number;
}

// User
export interface UserWithRole {
    id: string;
    email: string;
    name: string;
    role_id: string;
    role_slug: string;
    role_name: string;
}

// Role
export interface Role {
    id: string;
    name: string;
    slug: string;
    description: string | null;
    created_at: string;
}

// Permission
export interface Permission {
    id: string;
    name: string;
    description: string | null;
    resource: string;
    action: string;
    created_at: string;
}

// Post
export interface Post {
    id: string;
    title: string;
    slug: string;
    content: string;
    excerpt: string | null;
    status: PostStatus;
    author: Author | null;
    category: Category | null;
    tags: Tag[];
    created_at: string;
    updated_at: string;
}

export interface PostListItem {
    id: string;
    title: string;
    slug: string;
    excerpt: string | null;
    status: PostStatus;
    author_id: string;
    author_name: string | null;
    category_id: string | null;
    category_name: string | null;
    created_at: string;
}

export type PostStatus = 'draft' | 'published' | 'archived';

export interface Author {
    id: string;
    name: string;
    email: string;
}

// Category
export interface Category {
    id: string;
    name: string;
    slug: string;
    description: string | null;
    created_at: string;
    updated_at: string;
}

export interface CategoryWithCount extends Category {
    post_count: number;
}

// Tag
export interface Tag {
    id: string;
    name: string;
    slug: string;
    created_at: string;
}

export interface TagWithCount extends Tag {
    post_count: number;
}

// API Response wrapper
export interface ApiResponse<T> {
    success: boolean;
    data: T;
    error?: string;
    meta?: Meta;
}

export interface Meta {
    page: number;
    per_page: number;
    total: number;
    total_pages: number;
}

export interface MessageResponse {
    message: string;
}
