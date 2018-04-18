package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;

import java.nio.IntBuffer;
import java.util.HashMap;
import java.util.Map;

/**
 * Created by fraayala19 on 3/15/18.
 */
public class OpenGLTextureManager implements Disposable{
    private Map<String, GLTexture> textures;

    public OpenGLTextureManager() {
        textures = new HashMap<String, GLTexture>();
    }

    public GLTexture addTexture(FileHandle handle){
        if(!textures.containsKey(handle.path())){
            textures.put(handle.path(),new GLTexture(handle));
        }
        return textures.get(handle.path());
    }


    public int loadCubeMap(FileHandle[] handles){
        Pixmap[] pixmaps = new Pixmap[handles.length];

        for(int i = 0; i < pixmaps.length; i++){
            pixmaps[i] = new Pixmap(handles[i]);
        }

        int texture = Gdx.gl.glGenTexture();
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_CUBE_MAP, texture);

        for(int i = 0; i < pixmaps.length; i++){
            Gdx.gl.glTexImage2D(Gdx.gl.GL_TEXTURE_CUBE_MAP_POSITIVE_X + i, 0,
                    pixmaps[i].getGLInternalFormat(), pixmaps[i].getWidth(),
                    pixmaps[i].getHeight(), 0, pixmaps[i].getGLFormat(), pixmaps[i].getGLType(),
                    pixmaps[i].getPixels());
        }

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl.GL_TEXTURE_MIN_FILTER, Gdx.gl.GL_LINEAR);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl.GL_TEXTURE_MAG_FILTER, Gdx.gl.GL_LINEAR);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl.GL_TEXTURE_WRAP_S, Gdx.gl.GL_CLAMP_TO_EDGE);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl.GL_TEXTURE_WRAP_T, Gdx.gl.GL_CLAMP_TO_EDGE);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_CUBE_MAP, Gdx.gl30.GL_TEXTURE_WRAP_R, Gdx.gl.GL_CLAMP_TO_EDGE);

        for(int i = 0; i < pixmaps.length; i++){
            pixmaps[i].dispose();
        }

        return texture;

    }


    @Override
    public void dispose() {
        for(GLTexture i : textures.values()){
            i.dispose();
        }
    }
}
