package com.mygdx.game;

import com.badlogic.gdx.ApplicationAdapter;
import com.badlogic.gdx.Game;
import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.kotcrab.vis.ui.VisUI;

public class Boot extends Game {
	
	@Override
	public void create () {
		Gdx.graphics.setResizable(true);
		Gdx.graphics.setWindowedMode(800,600);
		Gdx.graphics.setTitle("Orbit Simulator");
		Gdx.graphics.setVSync(true);

		VisUI.load();

		setScreen(new SimulationScreen(this));
	}

	@Override
	public void dispose () {
	}
}
