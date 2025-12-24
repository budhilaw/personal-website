import { Card } from '../components/ui';

export function AboutPage() {
    return (
        <div className="container-page max-w-4xl">
            <h1 className="text-3xl font-bold text-gray-900 mb-8">About Me</h1>

            <Card className="mb-8">
                <div className="flex flex-col md:flex-row gap-8">
                    <div className="w-32 h-32 bg-gradient-to-br from-primary-400 to-primary-600 rounded-full flex-shrink-0 mx-auto md:mx-0" />
                    <div>
                        <h2 className="text-2xl font-semibold text-gray-900 mb-2">Hello! 👋</h2>
                        <p className="text-gray-600 leading-relaxed">
                            I'm a software developer passionate about building great products and sharing
                            knowledge with the community. This website serves as my digital garden where
                            I document my learning journey and share insights about software development.
                        </p>
                    </div>
                </div>
            </Card>

            <div className="grid md:grid-cols-2 gap-6">
                <Card>
                    <h3 className="text-lg font-semibold text-gray-900 mb-4">Skills</h3>
                    <div className="flex flex-wrap gap-2">
                        {['Rust', 'TypeScript', 'React', 'PostgreSQL', 'Redis', 'Docker'].map((skill) => (
                            <span
                                key={skill}
                                className="px-3 py-1 bg-primary-50 text-primary-700 rounded-full text-sm"
                            >
                                {skill}
                            </span>
                        ))}
                    </div>
                </Card>

                <Card>
                    <h3 className="text-lg font-semibold text-gray-900 mb-4">Get in Touch</h3>
                    <ul className="space-y-2 text-gray-600">
                        <li className="flex items-center gap-2">
                            <span className="text-gray-400">📧</span>
                            <a href="mailto:hello@example.com" className="hover:text-primary-600">
                                hello@example.com
                            </a>
                        </li>
                        <li className="flex items-center gap-2">
                            <span className="text-gray-400">🐙</span>
                            <a
                                href="https://github.com"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="hover:text-primary-600"
                            >
                                GitHub
                            </a>
                        </li>
                        <li className="flex items-center gap-2">
                            <span className="text-gray-400">💼</span>
                            <a
                                href="https://linkedin.com"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="hover:text-primary-600"
                            >
                                LinkedIn
                            </a>
                        </li>
                    </ul>
                </Card>
            </div>
        </div>
    );
}
