import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { postService } from '../services';
import { Card, Button } from '../components/ui';
import type { Post } from '../types';

export function BlogPostPage() {
    const { slug } = useParams<{ slug: string }>();
    const [post, setPost] = useState<Post | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchPost = async () => {
            if (!slug) return;

            try {
                const data = await postService.getBySlug(slug);
                setPost(data);
            } catch (err) {
                setError(err instanceof Error ? err.message : 'Failed to load post');
            } finally {
                setIsLoading(false);
            }
        };

        fetchPost();
    }, [slug]);

    if (isLoading) {
        return (
            <div className="container-page">
                <div className="flex items-center justify-center py-20">
                    <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
                </div>
            </div>
        );
    }

    if (error || !post) {
        return (
            <div className="container-page">
                <Card className="text-center py-12">
                    <p className="text-red-600 mb-4">{error || 'Post not found'}</p>
                    <Link to="/blog">
                        <Button variant="outline">Back to Blog</Button>
                    </Link>
                </Card>
            </div>
        );
    }

    return (
        <article className="container-page max-w-4xl">
            {/* Back link */}
            <Link
                to="/blog"
                className="inline-flex items-center text-gray-500 hover:text-gray-700 mb-8"
            >
                <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
                </svg>
                Back to Blog
            </Link>

            {/* Post header */}
            <header className="mb-8">
                <h1 className="text-4xl font-bold text-gray-900 mb-4">{post.title}</h1>
                <div className="flex items-center gap-4 text-gray-500">
                    {post.author && <span>By {post.author.name}</span>}
                    {post.category && (
                        <span className="px-2 py-1 bg-primary-100 text-primary-700 rounded-full text-sm">
                            {post.category.name}
                        </span>
                    )}
                    <span>{new Date(post.created_at).toLocaleDateString()}</span>
                </div>
                {post.tags.length > 0 && (
                    <div className="flex flex-wrap gap-2 mt-4">
                        {post.tags.map((tag) => (
                            <span
                                key={tag.id}
                                className="px-2 py-1 bg-gray-100 text-gray-600 rounded text-sm"
                            >
                                #{tag.name}
                            </span>
                        ))}
                    </div>
                )}
            </header>

            {/* Post content */}
            <div className="prose prose-lg max-w-none">
                <div dangerouslySetInnerHTML={{ __html: post.content }} />
            </div>
        </article>
    );
}
