package com.mygdx.game;

import com.badlogic.gdx.Game;
import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.assets.AssetManager;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.BitmapFont;
import com.kotcrab.vis.ui.VisUI;
import com.mygdx.game.playback.PlayBackScreen;
import com.mygdx.game.simulation.SimulationScreen;

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
		Gdx.graphics.setWindowedMode(1280,720);
		Gdx.graphics.setTitle("Astraria");
		Gdx.graphics.setVSync(true);

		manager = new AssetManager();
		manager.load("Euclid10.fnt", BitmapFont.class);
		manager.load("Star_texture_2.png", Texture.class);
		manager.load("particle.png", Texture.class);
		manager.load("indicator.png", Texture.class);
		manager.load("icons/list.png", Texture.class);
		manager.load("icons/info.png", Texture.class);
		manager.load("icons/timer.png", Texture.class);
		manager.load("icons/target.png", Texture.class);
		manager.load("icons/add.png", Texture.class);
		manager.load("logo/ASTRARIA LOGO-01.png", Texture.class);
		manager.load("stars_milky_way.jpg", Texture.class);
		manager.finishLoading();

		VisUI.load();

		if(arg.length>0) {
			String[] termination = arg[arg.length-1].split("\\.");
			System.out.println(termination.length);
			if (termination[termination.length-1].equals("txt")) {
				setScreen(new SimulationScreen(this, arg[arg.length-1]));
			} else if (termination[termination.length-1].equals("nbd")) {
				setScreen(new PlayBackScreen(this, arg[arg.length-1]));
			}
		} else {
			setScreen(new TitleScreen(this));
		}


	}

	@Override
	public void dispose () {
	}
}
