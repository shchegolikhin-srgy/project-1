import axios from 'axios';

const API_URL = process.env.REACT_APP_API_URL;

export const register = async (username, email, password) => {
  const response = await axios.post(`${API_URL}/api/auth/register`, {
    username,
    email,
    password,
  });
  return response.data;
};

export const login = async (username, password) => {
  const response = await axios.post(`${API_URL}/api/auth/login`, {
    username,
    password,
  });
  return response.data;
};

export const refreshToken = async (refreshToken) => {
  const response = await axios.post(`${API_URL}/api/auth/refresh`, {
    refreshToken,
  });
  return response.data;
};
