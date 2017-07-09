const axios = require('axios')

interface AxiosData {
  data: any
  status: number
}

export function getPosts() {
  return axios.get('/posts').then((d: AxiosData) => d.data)
}

export function getPost(id: number) {
  return axios.get(`/get_post/${id}`).then((d: AxiosData) => d.data)
}
