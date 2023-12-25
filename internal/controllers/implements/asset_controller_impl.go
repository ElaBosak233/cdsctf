package implements

import (
	"fmt"
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/gabriel-vasile/mimetype"
	"github.com/gin-gonic/gin"
	"mime/multipart"
	"net/http"
	"os"
)

type AssetControllerImpl struct {
	assetService services.AssetService
}

func NewAssetControllerImpl(appService *services.AppService) controllers.AssetController {
	return &AssetControllerImpl{
		assetService: appService.AssetService,
	}
}

// GetUserAvatarList
// @Summary 获取拥有头像的用户列表
// @Description 获取拥有头像的用户列表
// @Tags 资源
// @Accept json
// @Produce json
// @Router /api/assets/users/avatar/ [get]
func (c *AssetControllerImpl) GetUserAvatarList(ctx *gin.Context) {
	res, _ := c.assetService.GetUserAvatarList()
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": res,
	})
}

// FindUserAvatarByUserId
// @Summary 通过用户 Id 获取用户头像
// @Description 通过用户 Id 获取用户头像
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "用户 Id"
// @Router /api/assets/users/avatar/{id} [get]
func (c *AssetControllerImpl) FindUserAvatarByUserId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("./assets/users/avatar/%s", id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.Status(http.StatusNotFound)
	}
}

// CheckUserAvatarExistsByUserId
// @Summary 通过用户 Id 确认用户头像是否存在
// @Description 通过用户 Id 确认用户头像是否存在
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "用户 Id"
// @Router /api/assets/users/avatar/{id}/exists [get]
func (c *AssetControllerImpl) CheckUserAvatarExistsByUserId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("./assets/users/avatar/%s", id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// SetUserAvatarByUserId
// @Summary 通过用户 Id 设置用户头像
// @Description 通过用户 Id 设置用户头像
// @Tags 资源
// @Accept multipart/form-data
// @Param id path string true "用户 Id"
// @Param avatar formData file true "头像文件"
// @Router /api/assets/users/avatar/{id} [post]
func (c *AssetControllerImpl) SetUserAvatarByUserId(ctx *gin.Context) {
	id := ctx.Param("id")
	file, err := ctx.FormFile("avatar")
	if err != nil {
		ctx.Status(http.StatusBadRequest)
		return
	}
	mime, err := c.detectContentType(file)
	if !mime.Is("image/jpeg") && !mime.Is("image/png") && !mime.Is("image/gif") {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "格式不被允许",
		})
		return
	}
	err = ctx.SaveUploadedFile(file, fmt.Sprintf("./assets/users/avatar/%s", id))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeleteUserAvatarByUserId
// @Summary 通过用户 Id 删除用户头像
// @Description 通过用户 Id 删除用户头像
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "用户 Id"
// @Router /api/assets/users/avatar/{id} [delete]
func (c *AssetControllerImpl) DeleteUserAvatarByUserId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("./assets/users/avatar/%s", id)
	_, err := os.Stat(path)
	if err == nil {
		err = os.Remove(path)
		if err != nil {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusInternalServerError,
				"msg":  err.Error(),
			})
			return
		}
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// GetTeamAvatarList
// @Summary 获取拥有头像的团队列表
// @Description 获取拥有头像的团队列表
// @Tags 资源
// @Accept json
// @Produce json
// @Router /api/assets/teams/avatar/ [get]
func (c *AssetControllerImpl) GetTeamAvatarList(ctx *gin.Context) {
	res, _ := c.assetService.GetTeamAvatarList()
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": res,
	})
}

// FindTeamAvatarByTeamId
// @Summary 通过团队 Id 获取团队头像
// @Description 通过团队 Id 获取团队头像
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "团队 Id"
// @Router /api/assets/teams/avatar/{id} [get]
func (c *AssetControllerImpl) FindTeamAvatarByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("./assets/teams/avatar/%s", id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.Status(http.StatusNotFound)
	}
}

// CheckTeamAvatarExistsByTeamId
// @Summary 通过团队 Id 确认团队头像是否存在
// @Description 通过团队 Id 确认团队头像是否存在
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "团队 Id"
// @Router /api/assets/teams/avatar/{id}/exists [get]
func (c *AssetControllerImpl) CheckTeamAvatarExistsByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("./assets/teams/avatar/%s", id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// SetTeamAvatarByTeamId
// @Summary 通过团队 Id 设置团队头像
// @Description 通过团队 Id 设置团队头像
// @Tags 资源
// @Accept multipart/form-data
// @Param id path string true "团队 Id"
// @Param avatar formData file true "头像文件"
// @Router /api/assets/teams/avatar/{id} [post]
func (c *AssetControllerImpl) SetTeamAvatarByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	file, err := ctx.FormFile("avatar")
	if err != nil {
		ctx.Status(http.StatusBadRequest)
		return
	}
	mime, err := c.detectContentType(file)
	if !mime.Is("image/jpeg") && !mime.Is("image/png") && !mime.Is("image/gif") {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "格式不被允许",
		})
		return
	}
	err = ctx.SaveUploadedFile(file, fmt.Sprintf("./assets/teams/avatar/%s", id))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// FindGameCoverByGameId
// @Summary 通过比赛 Id 获取比赛封面
// @Description 通过比赛 Id 获取比赛封面
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "比赛 Id"
// @Router /api/assets/games/cover/{id} [get]
func (c *AssetControllerImpl) FindGameCoverByGameId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("./assets/games/cover/%s", id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.Status(http.StatusNotFound)
	}
}

// SetGameCoverByGameId
// @Summary 通过比赛 Id 设置比赛封面
// @Description 通过比赛 Id 设置比赛封面
// @Tags 资源
// @Accept multipart/form-data
// @Param id path string true "比赛 Id"
// @Param avatar formData file true "封面文件"
// @Router /api/assets/games/cover/{id} [post]
func (c *AssetControllerImpl) SetGameCoverByGameId(ctx *gin.Context) {
	id := ctx.Param("id")
	file, err := ctx.FormFile("avatar")
	if err != nil {
		ctx.Status(http.StatusBadRequest)
		return
	}
	mime, err := c.detectContentType(file)
	if !mime.Is("image/jpeg") && !mime.Is("image/png") && !mime.Is("image/gif") {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "格式不被允许",
		})
		return
	}
	err = ctx.SaveUploadedFile(file, fmt.Sprintf("./assets/games/cover/%s", id))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// FindGameWriteUpByTeamId
// @Summary 通过团队 Id 获取比赛 Writeup
// @Description 通过团队 Id 获取比赛 Writeup
// @Tags 资源
// @Accept json
// @Produce json
// @Param id path string true "团队 Id"
// @Router /api/assets/games/writeups/{id} [get]
func (c *AssetControllerImpl) FindGameWriteUpByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("./assets/games/writeups/%s.pdf", id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.Status(http.StatusNotFound)
	}
}

func (c *AssetControllerImpl) detectContentType(file *multipart.FileHeader) (mime *mimetype.MIME, err error) {
	src, err := file.Open()
	if err != nil {
		return nil, err
	}
	defer func(src multipart.File) {
		_ = src.Close()
	}(src)
	buffer := make([]byte, 512)
	_, err = src.Read(buffer)
	if err != nil {
		return nil, err
	}
	return mimetype.Detect(buffer), nil
}
