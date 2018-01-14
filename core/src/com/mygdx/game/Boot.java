package com.mygdx.game;

import com.badlogic.gdx.ApplicationAdapter;
import com.badlogic.gdx.Game;
import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.assets.AssetManager;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.BitmapFont;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.kotcrab.vis.ui.VisUI;
import com.mygdx.game.playback.PlayBackScreen;

public class Boot extends Game {

	public static AssetManager manager;
	private String[] arg;
	

	public Boot(String[] arg) {
		super();
		this.arg = arg;
	}

	@Override
	public void create () {
		Gdx.graphics.setResizable(true);
		Gdx.graphics.setWindowedMode(800,600);
		Gdx.graphics.setTitle("Orbit Simulator");
		Gdx.graphics.setVSync(true);

		manager = new AssetManager();
		manager.load("comicsans.fnt", BitmapFont.class);
		manager.load("Star_texture_2.png", Texture.class);
		manager.finishLoading();

		VisUI.load();

		if(arg[0].equals("sim")){
			setScreen(new SimulationScreen(this, arg[1]));
		} else if (arg[0].equals("play")){
			setScreen(new PlayBackScreen(this,arg[1]));
		}


	}

	@Override
	public void dispose () {
	}
}
