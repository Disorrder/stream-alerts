import axios from "axios";

export const api = axios.create({
  baseURL: "http://localhost:6967",
});

api.interceptors.response.use(
  (response) => {
    return response;
  },
  async (error) => {
    const url = error.config.url;

    if (url?.startsWith("/twitch")) {
      const { status, data } = error.response;
      if (status === 401 || data === "token is not authorized for use") {
        await api.post("/twitch/refresh");
        return api.request(error.config);
      }
    }
    return Promise.reject(error);
  },
);
