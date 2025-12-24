import { api } from './api';
import type { PostListItem, Post, Meta } from '../types';

export interface PostQueryParams {
    page?: number;
    per_page?: number;
    status?: string;
    category_id?: string;
    tag_id?: string;
    search?: string;
}

export const postService = {
    async list(params: PostQueryParams = {}): Promise<{ posts: PostListItem[]; meta?: Meta }> {
        const searchParams = new URLSearchParams();

        if (params.page) searchParams.set('page', params.page.toString());
        if (params.per_page) searchParams.set('per_page', params.per_page.toString());
        if (params.status) searchParams.set('status', params.status);
        if (params.category_id) searchParams.set('category_id', params.category_id);
        if (params.tag_id) searchParams.set('tag_id', params.tag_id);
        if (params.search) searchParams.set('search', params.search);

        const query = searchParams.toString();
        const endpoint = `/posts${query ? `?${query}` : ''}`;

        const response = await api.get<PostListItem[]>(endpoint);
        return { posts: response.data, meta: response.meta };
    },

    async getBySlug(slug: string): Promise<Post> {
        const response = await api.get<Post>(`/posts/slug/${slug}`);
        return response.data;
    },

    async create(data: Partial<Post>): Promise<Post> {
        const response = await api.post<Post>('/posts', data);
        return response.data;
    },

    async update(id: string, data: Partial<Post>): Promise<Post> {
        const response = await api.put<Post>(`/posts/${id}`, data);
        return response.data;
    },

    async delete(id: string): Promise<void> {
        await api.delete(`/posts/${id}`);
    },
};
