package com.mygdx.game.simulation.renderer;

/**
 * Created by fraayala19 on 4/12/18.
 */
public class Warehouse {
    private static OpenGLTextureManager openGLTextureManager;
    private static ModelManager modelManager;

    public static void init(){
        if(openGLTextureManager != null){
            openGLTextureManager.dispose();
        }
        if(modelManager!=null){
            modelManager.dispose();
        }

        openGLTextureManager = new OpenGLTextureManager();
        modelManager = new ModelManager(openGLTextureManager);
    }

    public static void dispose(){
        openGLTextureManager.dispose();
        modelManager.dispose();

        openGLTextureManager = null;
        modelManager = null;
    }

    public static OpenGLTextureManager getOpenGLTextureManager() {
        return openGLTextureManager;
    }

    public static ModelManager getModelManager() {
        return modelManager;
    }
}
