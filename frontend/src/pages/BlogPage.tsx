import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { postService } from '../services';
import { Card } from '../components/ui';
import type { PostListItem } from '../types';

export function BlogPage() {
    const [posts, setPosts] = useState<PostListItem[]>([]);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchPosts = async () => {
            try {
                const { posts } = await postService.list({ status: 'published' });
                setPosts(posts);
            } catch (err) {
                setError(err instanceof Error ? err.message : 'Failed to load posts');
            } finally {
                setIsLoading(false);
            }
        };

        fetchPosts();
    }, []);

    if (isLoading) {
        return (
            <div className="container-page">
                <div className="flex items-center justify-center py-20">
                    <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="container-page">
                <Card className="text-center py-12">
                    <p className="text-red-600">{error}</p>
                </Card>
            </div>
        );
    }

    return (
        <div className="container-page">
            <div className="mb-8">
                <h1 className="text-3xl font-bold text-gray-900">Blog</h1>
                <p className="mt-2 text-gray-600">Thoughts, tutorials, and updates</p>
            </div>

            {posts.length === 0 ? (
                <Card className="text-center py-12">
                    <p className="text-gray-500">No posts yet. Check back soon!</p>
                </Card>
            ) : (
                <div className="grid gap-6">
                    {posts.map((post) => (
                        <Link key={post.id} to={`/blog/${post.slug}`}>
                            <Card className="hover:shadow-md transition-shadow duration-200">
                                <div className="flex items-start justify-between">
                                    <div>
                                        <h2 className="text-xl font-semibold text-gray-900 hover:text-primary-600 transition-colors">
                                            {post.title}
                                        </h2>
                                        {post.excerpt && (
                                            <p className="mt-2 text-gray-600 line-clamp-2">{post.excerpt}</p>
                                        )}
                                        <div className="mt-4 flex items-center gap-4 text-sm text-gray-500">
                                            {post.author_name && <span>By {post.author_name}</span>}
                                            {post.category_name && (
                                                <span className="px-2 py-1 bg-gray-100 rounded-full text-xs">
                                                    {post.category_name}
                                                </span>
                                            )}
                                            <span>{new Date(post.created_at).toLocaleDateString()}</span>
                                        </div>
                                    </div>
                                </div>
                            </Card>
                        </Link>
                    ))}
                </div>
            )}
        </div>
    );
}
