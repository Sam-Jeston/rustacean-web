import * as React from "react";
import { getPost } from '../services/posts'

export class Post extends React.Component<any, {post: PostDef}> {
  constructor(props: any) {
    super(props)
    console.log(this.props)
    this.state = { post: {} as PostDef }
  }

  public async componentDidMount () {
    const post: PostDef = await getPost(1)
    this.setState({post})
  }

  render() {
    return (
      <div>
        <h1>Post {this.state.post.id}!</h1>
      </div>
    )
  }
}
