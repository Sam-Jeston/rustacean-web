import * as React from "react";

export class Home extends React.Component<any, undefined> {
  public navigate() {
    this.props.history.push('/some/path');
  }

  render() {
    return (
      <div className="container">
        <h1 className="title is-2">Rust on the Web</h1>
        <p>
          <a href="http://www.arewewebyet.org/">Are We Web Yet?</a> provides a great overview of available web technologies for Rust.
        </p>
        <br />
        <p>Here at <a href="https://rustontheweb.com/">Rust On The Web</a> I write articles about creating web applications with Rust, and all examples are built into the site!</p>
        <br />
        <div className="box" style={{cursor: 'pointer'}} onClick={() => this.navigate()}>
          <h2 className="title is-4">Hello</h2>
          <p>some blog content...</p>
        </div>
      </div>
    )
  }
}
