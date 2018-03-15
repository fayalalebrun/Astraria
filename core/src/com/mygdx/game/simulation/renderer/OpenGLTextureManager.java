package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;

import java.util.HashMap;
import java.util.Map;

/**
 * Created by fraayala19 on 3/15/18.
 */
public class OpenGLTextureManager {
    Map<String, Integer> textures;

    public OpenGLTextureManager() {
        textures = new HashMap<String, Integer>();
    }

    public int addTexture(String path){
        if(!textures.containsKey(path)){
            textures.put(path,loadTexture(new FileHandle(path)));
        }

        return textures.get(path);
    }

    private int loadTexture(FileHandle handle){
        int texture = Gdx.gl.glGenTexture();
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, texture);

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_WRAP_S, Gdx.gl.GL_REPEAT);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_WRAP_T, Gdx.gl.GL_REPEAT);

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_MIN_FILTER, Gdx.gl.GL_LINEAR);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_MAG_FILTER, Gdx.gl.GL_LINEAR);

        Texture tex = new Texture(handle);

        tex.getTextureData().prepare();


        Pixmap pixmap = tex.getTextureData().consumePixmap();

        Gdx.gl.glTexImage2D(Gdx.gl.GL_TEXTURE_2D, 0, pixmap.getGLInternalFormat(), pixmap.getWidth(),
                pixmap.getHeight(), 0, pixmap.getGLFormat(), pixmap.getGLType(),
                pixmap.getPixels());
        Gdx.gl.glGenerateMipmap(Gdx.gl.GL_TEXTURE_2D);
        pixmap.dispose();
        tex.dispose();
        return texture;
    }
}
