import os
from flask import Flask, jsonify, session
from authlib.integrations.flask_client import OAuth
from flask import url_for, redirect
from dotenv import load_dotenv
load_dotenv()


def create_app(test_config=None):
    from . import db
    from . import auth
    from . import blog

    # create and configure the app
    app = Flask(__name__, instance_relative_config=True)
    app.config.from_mapping(
        SECRET_KEY='dev',
        DATABASE=os.path.join(app.instance_path, 'flaskr.sqlite'),
    )
    app.secret_key = 'sosecreet'
    oauth = OAuth(app)
    google = oauth.register(
        name=os.getenv('name'),
        client_id= os.getenv('client_id'),
        client_secret=os.getenv('client_secret'),
        access_token_url=os.getenv('access_token_url'),
        access_token_params=None,
        authorize_url=os.getenv('authorize_url'),
        authorize_params=None,
        api_base_url=os.getenv('api_base_url'),
        client_kwargs=os.getenv('client_kwargs'),
    )
    if test_config is None:
        # load the instance config, if it exists, when not testing
        app.config.from_pyfile('config.py', silent=True)
    else:
        # load the test config if passed in
        app.config.from_mapping(test_config)

    # ensure the instance folder exists
    try:
        os.makedirs(app.instance_path)
    except OSError:
        pass

    # a simple page that says hello
    @app.route('/hello')
    def hello():
        email = dict(session).get('email', None)
        return f'hello {email}'

    @app.route('/login')
    def login():
        google = oauth.create_client('google')
        redirect_uri = url_for('authorize', _external=True)
        return google.authorize_redirect(redirect_uri)

    @app.route('/authorize')
    def authorize():
        google = oauth.create_client('google')

        token = google.authorize_access_token()
        resp = google.get('userinfo', token=token)
        resp.raise_for_status()
        profile = resp.json()
        session['email'] = profile['email']
        # do something with the token and profile
        return redirect('/')

    db.init_app(app)
    app.register_blueprint(auth.AuthenticationBlueprint)
    app.register_blueprint(blog.blog_blueprint)
    app.add_url_rule('/', endpoint='index')
    return app
