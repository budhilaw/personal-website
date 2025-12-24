import { Link } from 'react-router-dom';
import { Button, Card } from '../components/ui';

export function HomePage() {
    return (
        <div>
            {/* Hero Section */}
            <section className="bg-gradient-to-br from-primary-50 to-white py-20">
                <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div className="text-center">
                        <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-6">
                            Welcome to My Personal Website
                        </h1>
                        <p className="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
                            A place where I share my thoughts, projects, and experiences in software development.
                        </p>
                        <div className="flex items-center justify-center gap-4">
                            <Link to="/blog">
                                <Button size="lg">Read Blog</Button>
                            </Link>
                            <Link to="/about">
                                <Button variant="outline" size="lg">
                                    About Me
                                </Button>
                            </Link>
                        </div>
                    </div>
                </div>
            </section>

            {/* Features Section */}
            <section className="py-16">
                <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 className="text-3xl font-bold text-center text-gray-900 mb-12">
                        What You'll Find Here
                    </h2>
                    <div className="grid md:grid-cols-3 gap-8">
                        <Card>
                            <div className="text-center">
                                <div className="w-12 h-12 bg-primary-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                                    <svg
                                        className="w-6 h-6 text-primary-600"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            strokeLinecap="round"
                                            strokeLinejoin="round"
                                            strokeWidth={2}
                                            d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                                        />
                                    </svg>
                                </div>
                                <h3 className="text-lg font-semibold text-gray-900 mb-2">Blog Posts</h3>
                                <p className="text-gray-600">
                                    Articles about software development, best practices, and lessons learned.
                                </p>
                            </div>
                        </Card>

                        <Card>
                            <div className="text-center">
                                <div className="w-12 h-12 bg-primary-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                                    <svg
                                        className="w-6 h-6 text-primary-600"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            strokeLinecap="round"
                                            strokeLinejoin="round"
                                            strokeWidth={2}
                                            d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
                                        />
                                    </svg>
                                </div>
                                <h3 className="text-lg font-semibold text-gray-900 mb-2">Projects</h3>
                                <p className="text-gray-600">
                                    Showcase of personal and open source projects I've worked on.
                                </p>
                            </div>
                        </Card>

                        <Card>
                            <div className="text-center">
                                <div className="w-12 h-12 bg-primary-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                                    <svg
                                        className="w-6 h-6 text-primary-600"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            strokeLinecap="round"
                                            strokeLinejoin="round"
                                            strokeWidth={2}
                                            d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                                        />
                                    </svg>
                                </div>
                                <h3 className="text-lg font-semibold text-gray-900 mb-2">About Me</h3>
                                <p className="text-gray-600">
                                    Learn more about my background, skills, and professional journey.
                                </p>
                            </div>
                        </Card>
                    </div>
                </div>
            </section>
        </div>
    );
}
