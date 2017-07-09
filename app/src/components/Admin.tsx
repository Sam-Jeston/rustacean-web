import * as React from "react";
import { createPost } from '../services/posts'

export class Admin extends React.Component<any, any> {
  constructor(props: any) {
    super(props)
    this.createPost = this.createPost.bind(this)
    this.handleInputChange = this.handleInputChange.bind(this)
  }

  public createPost (e: any) {
    console.log('aaa')
    e.preventDefault()
    // createPost('post again', 'with a caption', 'and a body')
    // Okay from here we submit with state
    console.log(this.state)
  }

  public handleInputChange(event: any) {
    const target = event.target
    const value = target.type === 'checkbox' ? target.checked : target.value
    const name = target.name

    this.setState({
      [name]: value
    });
  }

  render() {
    return (
      <div className="container">
        <h1 className="title is-2">Create a Post</h1>
        <form>
          <div className="field">
            <label className="label">Title</label>
            <p className="control">
              <input className="input" name="title" type="text" placeholder="Title" onChange={this.handleInputChange} />
            </p>
          </div>
          <div className="field">
            <label className="label">Caption</label>
            <p className="control">
              <input className="input" name="caption" type="text" placeholder="Caption" onChange={this.handleInputChange} />
            </p>
          </div>
          <input className="button is-primary" type="submit" value="Submit" onClick={this.createPost} onSubmit={this.createPost}/>
        </form>
      </div>
    )
  }
}
