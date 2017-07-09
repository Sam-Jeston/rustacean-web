import * as React from "react";
import { getPost, createPost } from '../services/posts'

export class Post extends React.Component<any, {post: PostDef, postId: number}> {
  constructor(props: any) {
    super(props)
    const pathLen = this.props.location.pathname.length
    this.state = { post: {} as PostDef, postId: this.props.location.pathname[pathLen - 1] }
  }

  public async componentDidMount () {
    const post: PostDef = await getPost(this.state.postId)
    this.setState({post})
  }

  public createPost () {
    createPost('post again', 'with a caption', 'and a body')
  }

  render() {
    return (
      <div className="container">
        <h1 className="title is-2">{this.state.post.title}</h1>
        <p>{this.state.post.body}</p>
        <button className="button is-primary" onClick={() => this.createPost()}>create dummy post</button>
      </div>
    )
  }
}
