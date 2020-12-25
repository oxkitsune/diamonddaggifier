import './App.css';
import EMPTY_CAPE from './empty_cape.png';
import 'bootstrap/dist/css/bootstrap.min.css';
import { Component } from 'react';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form'
import Col from 'react-bootstrap/Col'
import ReactSkinview3d from 'react-skinview3d'
const API = process.env.DD_API_URL || "http://127.0.0.1:80";

class App extends Component {

  constructor(props) {
    super(props);
    this.state = {
      name: 'DiamondDagger590',
      uuid: 'b94b32a409e84378905b0df7805916c1',
      busy: false
    }

    this.model = <ReactSkinview3d
      capeUrl={EMPTY_CAPE}
      skinUrl={API + "/api/skin/" + this.state.uuid}
      height="250"
      width="250"
      enableOrbitControls={false}
      onReady={(instance) => {
        this._modelInstance = instance;
        this._modelInstance.animations.add((player, time) => {
          player.rotation.y += .033;
        });
        this.updateSkin();
      }}

    />
    this.handleChange = this.handleChange.bind(this);
    this.handleKeyDown = this.handleKeyDown.bind(this);
    this.downloadSkin = this.downloadSkin.bind(this);
    this.uploadSkin = this.uploadSkin.bind(this);
    this.diamonddaggify = this.diamonddaggify.bind(this);
  }

  handleChange(e) {
    this.setState({ name: e.target.value, uuid: 'NULL', busy: this.state.busy });
  }

  handleKeyDown(e) {
    if (e.key === 'Enter') {
      this.diamonddaggify(e);
    }
  }

  updateSkin() {

    let skinUrl = API + "/api/skin/" + this.state.uuid;
    fetch(skinUrl, {
      method: 'GET',
      mode: 'cors',
      headers: {
        'Content-Type': 'image/png'
      }
    })
      .then((result) => {
        if (result.status !== 200) {
          alert("Failed to get skin from API (might be an invalid name!): " + result.statusText);
          this.setState({
            name: this.state.name,
            uuid: 'NULL',
            busy: false
          });
        }
        else {
          this._modelInstance.loadSkin(skinUrl);
        }
      }, (error) => {
        alert("Failed to get skin from API: " + error);
        this.setState({
          name: this.state.name,
          uuid: 'NULL',
          busy: false
        });
      });
  }

  diamonddaggify(e) {
    e.preventDefault();

    this.setState({ name: this.state.name, uuid: 'NULL', busy: true })

    fetch(API + "/api/uuid/" + this.state.name, {
      method: 'GET',
      mode: 'cors',
      headers: {
        'Content-Type': 'application/json',
      }
    })
      .then(res => res.json())
      .then(
        (result) => {


          if (Object.keys(result).includes("error")) {
            window.alert(result.error);

            this.setState({
              name: this.state.name,
              uuid: 'NULL',
              busy: false
            });

            return;
          }

          this.setState({
            name: this.state.name,
            uuid: result.uuid,
            busy: false
          });
          this.updateSkin();
        },
        (error) => {
          window.alert(error.error);
          this.setState({
            name: this.state.name,
            uuid: 'NULL',
            busy: false
          });
        })
  }

  downloadSkin(e) {
    e.preventDefault();
    window.location.href = API + "/api/skin/" + this.state.uuid + "/download";
  }

  uploadSkin(e) {
    e.preventDefault();
    window.location.href = "https://www.minecraft.net/profile/skin/remote?url=" + API + "/api/skin/" + this.state.uuid + "?v302";
  }

  render() {

    return (
      <div className="App">
        <h1 style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>DiamondDaggify your skin!</h1>
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '25vh' }}>
          {this.model}
        </div>
        <div>
          <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
            <Form.Group>
              <Form.Row className="align-items-center">
                <Col xs="auto">
                  <Form.Control className="text-center" onKeyDown={this.handleKeyDown} onChange={this.handleChange} type="username" placeholder="Enter username" />
                </Col>
                <Col xs='auto'>
                  <Button variant="primary" onClick={this.diamonddaggify} disabled={this.state.busy}>DiamondDaggify!</Button>{' '}
                </Col>
              </Form.Row>
            </Form.Group>
          </div>
          <Button variant="secondary" onClick={this.downloadSkin} disabled={this.state.uuid === 'NULL' ? true : false}>Download</Button>{' '}
          <Button variant="success" onClick={this.uploadSkin} disabled={this.state.uuid === 'NULL' ? true : false}>Upload to minecraft.net</Button>{' '}
        </div>
      </div >
    );
  }
}

export default App;