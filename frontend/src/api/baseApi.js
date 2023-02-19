import axios from 'axios';

function createAxiosInstance() {
    const instance = axios.create({
        baseURL: 'http://localhost:8000',
        timeout: 1000,
    });
    instance.interceptors.request.use(
        (req) => {
            // use authentication?
            return req;
        },
        (error) => {
            console.log('[API ERROR]', error);
            return Promise.reject(error);
        }
    );
    
    return instance;
};

export default createAxiosInstance;