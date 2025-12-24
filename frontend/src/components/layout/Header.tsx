import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../../context';
import { Button } from '../ui';

export function Header() {
    const { user, isAuthenticated, logout } = useAuth();
    const navigate = useNavigate();

    const handleLogout = async () => {
        await logout();
        navigate('/');
    };

    return (
        <header className="bg-white border-b border-gray-200">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div className="flex items-center justify-between h-16">
                    {/* Logo */}
                    <Link to="/" className="flex items-center">
                        <span className="text-xl font-bold text-primary-600">Personal Website</span>
                    </Link>

                    {/* Navigation */}
                    <nav className="hidden md:flex items-center space-x-8">
                        <Link to="/" className="text-gray-600 hover:text-gray-900 transition-colors">
                            Home
                        </Link>
                        <Link to="/blog" className="text-gray-600 hover:text-gray-900 transition-colors">
                            Blog
                        </Link>
                        <Link to="/about" className="text-gray-600 hover:text-gray-900 transition-colors">
                            About
                        </Link>
                    </nav>

                    {/* Auth buttons */}
                    <div className="flex items-center space-x-4">
                        {isAuthenticated ? (
                            <>
                                <span className="text-sm text-gray-600">
                                    Hi, <span className="font-medium">{user?.name}</span>
                                </span>
                                {user?.role_slug === 'admin' && (
                                    <Link to="/admin">
                                        <Button variant="outline" size="sm">
                                            Dashboard
                                        </Button>
                                    </Link>
                                )}
                                <Button variant="ghost" size="sm" onClick={handleLogout}>
                                    Logout
                                </Button>
                            </>
                        ) : (
                            <Link to="/login">
                                <Button size="sm">Login</Button>
                            </Link>
                        )}
                    </div>
                </div>
            </div>
        </header>
    );
}
