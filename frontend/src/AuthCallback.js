import React, { useEffect } from 'react';
import { useSearchParams, useNavigate } from 'react-router-dom';
import { linuxdoAuthorized } from './api';
import { checkLogin } from './api';

const AuthCallback = ({ setIsAuthenticated, setUser }) => {
    const navigate = useNavigate();
    const [searchParams] = useSearchParams();  // 获取 URL 中的参数

    useEffect(() => {
        const authorizeUser = async () => {
            const params = {};
            searchParams.forEach((value, key) => {
                params[key] = value;
            });

            try {
                const response = await linuxdoAuthorized(params);  // 传递参数到后端
                if (response.status === 200) {
                    const response = await checkLogin();
                    if (response.data.code === 0) {
                        setIsAuthenticated(true); // 更新登录状态
                        setUser(response.data.data); // 更新用户信息
                        localStorage.setItem('isAuthenticated', true);
                        sessionStorage.setItem('user', JSON.stringify(response.data.data));
                    } else {
                        setIsAuthenticated(false);
                    }
                    navigate('/');
                }
            } catch (error) {
                console.error('Authorization failed', error);
            }
        };

        authorizeUser();
    }, [searchParams, navigate, setIsAuthenticated, setUser]);

    return <div>Authorizing...</div>;
};

export default AuthCallback;
