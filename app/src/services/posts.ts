const axios = require('axios')

interface AxiosData {
  data: any
  status: number
}

export function getPosts() {
  return axios.get('/posts').then((d: AxiosData) => d.data)
}

export function getPost(id: number) {
  return axios.get(`/post/${id}`).then((d: AxiosData) => d.data)
}

export function createPost(title: string, caption: string, body: string) {
  return axios.post(`/post`, {
    title,
    caption,
    body,
    created_at: new Date(),
    updated_at: new Date()
  })
}
